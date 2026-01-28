//! Tauri commands (IPC layer) - THIN orchestration
//!
//! This module contains all Tauri commands that serve as the interface
//! between the frontend and backend. Commands should be thin wrappers
//! that delegate to domain modules.

use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::mpsc;

use crate::audio::{self, RecordingHandle};
use crate::error::AppError;
use crate::system::shutdown;
use crate::transcription::{get_model_path, transcribe_audio, WhisperModel, WhisperState};

/// State global pour le stream audio actif
/// RecordingHandle est Send + Sync car il utilise des channels
pub struct AudioState {
    pub recording: Mutex<Option<RecordingHandle>>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            recording: Mutex::new(None),
        }
    }
}

/// Returns the application version from tauri.conf.json.
///
/// # Returns
/// The version string (e.g., "0.1.0") or "0.0.0" if not configured.
#[tauri::command]
pub fn get_version(app: AppHandle) -> Result<String, AppError> {
    let version = app
        .config()
        .version
        .clone()
        .unwrap_or_else(|| "0.0.0".to_string());
    Ok(version)
}

/// Requests a graceful shutdown of the application.
///
/// This command performs cleanup (temp files, active recordings) before
/// exiting the application. Called via Ctrl+Q or menu "Quitter".
///
/// # Errors
/// Returns `AppError` if cleanup fails.
#[tauri::command]
pub fn request_quit(app: AppHandle) -> Result<(), AppError> {
    // Perform graceful shutdown cleanup
    shutdown::graceful_shutdown()?;

    // Exit the application
    app.exit(0);

    Ok(())
}

/// Démarre l'enregistrement audio
///
/// # Errors
/// - `MicrophoneNotFound` si aucun microphone détecté
/// - `MicrophoneAccessDenied` si permissions insuffisantes
/// - `RecordingInterrupted` si enregistrement déjà en cours
#[tauri::command]
pub fn start_recording(
    state: State<'_, AudioState>,
    app: AppHandle,
) -> Result<(), AppError> {
    let mut recording_guard = state
        .recording
        .lock()
        .expect("Audio state lock poisoned - should never happen in single-threaded Tauri context");

    // Vérifier qu'un enregistrement n'est pas déjà en cours
    if recording_guard.is_some() {
        return Err(AppError::RecordingInterrupted);
    }

    // Créer channel pour waveform data (capacity 100 pour éviter backpressure)
    let (tx, mut rx) = mpsc::channel::<Vec<f32>>(100);

    // Démarrer l'enregistrement
    let recording_handle = audio::start_recording(Some(tx))?;
    *recording_guard = Some(recording_handle);

    // Spawn task pour émettre events waveform vers frontend
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(samples) = rx.recv().await {
            // Émettre event waveform-data vers frontend
            let _ = app_handle.emit("waveform-data", samples);
        }
    });

    // Émettre event recording-started
    let _ = app.emit("recording-started", ());

    Ok(())
}

/// Arrête l'enregistrement et sauvegarde le fichier WAV
///
/// # Returns
/// Chemin du fichier WAV créé
///
/// # Errors
/// - `RecordingInterrupted` si aucun enregistrement en cours
/// - `IoError` si échec écriture fichier
#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AudioState>,
    app: AppHandle,
) -> Result<String, AppError> {
    // Récupérer le handle d'enregistrement
    let recording_handle = {
        let mut recording_guard = state
            .recording
            .lock()
            .expect("Audio state lock poisoned - should never happen in single-threaded Tauri context");

        recording_guard
            .take()
            .ok_or(AppError::RecordingInterrupted)?
    };

    // Arrêter et récupérer les samples avec le sample rate réel (async)
    let result = recording_handle.stop().await?;

    // Sauvegarder en WAV avec le sample rate réel utilisé pendant la capture
    let wav_path = audio::save_wav(&result.samples, result.sample_rate)?;

    // Émettre event recording-stopped avec durée
    let duration_secs = result.samples.len() as f64 / result.sample_rate as f64;
    let _ = app.emit("recording-stopped", duration_secs);

    Ok(wav_path.to_string_lossy().to_string())
}

/// Payload for transcription progress events.
#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    percent: i32,
}

/// Payload for transcription complete events.
#[derive(Clone, serde::Serialize)]
struct TranscriptionPayload {
    text: String,
}

/// Lance la transcription de manière asynchrone.
///
/// Retourne immédiatement - résultat via événements:
/// - transcription-progress: { percent: 0-100 }
/// - transcription-complete: { text: "..." }
/// - error: { type: "...", message: "..." }
///
/// # Arguments
/// * `audio_path` - Chemin vers le fichier WAV à transcrire
///
/// # Errors
/// - `TranscriptionFailed` si le fichier audio n'existe pas
#[tauri::command]
pub async fn start_transcription(
    app: AppHandle,
    whisper_state: State<'_, WhisperState>,
    audio_path: String,
) -> Result<(), AppError> {
    let audio_path = PathBuf::from(&audio_path);

    // Vérifier que le fichier existe
    if !audio_path.exists() {
        return Err(AppError::TranscriptionFailed(format!(
            "Fichier audio introuvable: {}",
            audio_path.display()
        )));
    }

    // Clone les éléments nécessaires pour le spawn
    let model_arc = whisper_state.model.clone();
    let app_clone = app.clone();

    // Spawn async task pour ne pas bloquer
    tokio::spawn(async move {
        // Émettre progression initiale
        let _ = app_clone.emit("transcription-progress", ProgressPayload { percent: 0 });

        // Charger le modèle si nécessaire (lazy loading)
        let mut model_guard = model_arc.lock().await;
        if model_guard.is_none() {
            let _ = app_clone.emit("transcription-progress", ProgressPayload { percent: 5 });

            match get_model_path() {
                Ok(model_path) => match WhisperModel::load(&model_path) {
                    Ok(model) => {
                        println!("Model loaded successfully");
                        *model_guard = Some(model);
                    }
                    Err(e) => {
                        eprintln!("Failed to load model: {:?}", e);
                        let _ = app_clone.emit("error", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to get model path: {:?}", e);
                    let _ = app_clone.emit("error", e);
                    return;
                }
            }
        }

        let _ = app_clone.emit("transcription-progress", ProgressPayload { percent: 10 });

        // Transcription (cette partie est CPU-intensive)
        // Note: whisper-rs ne supporte pas les callbacks de progression natifs
        // On simule avec des étapes discrètes
        if let Some(ref model) = *model_guard {
            let _ = app_clone.emit("transcription-progress", ProgressPayload { percent: 20 });

            match transcribe_audio(model, &audio_path) {
                Ok(text) => {
                    let _ = app_clone.emit("transcription-progress", ProgressPayload { percent: 100 });
                    let _ = app_clone.emit(
                        "transcription-complete",
                        TranscriptionPayload { text },
                    );
                }
                Err(e) => {
                    eprintln!("Transcription failed: {:?}", e);
                    let _ = app_clone.emit("error", e);
                }
            }

            // NFR-SEC-1: Cleanup immédiat du fichier audio temporaire (privacy-first)
            if let Err(e) = std::fs::remove_file(&audio_path) {
                eprintln!("Warning: Failed to cleanup temp audio file: {:?}", e);
            } else {
                println!("Temp audio file cleaned up: {}", audio_path.display());
            }
        }
    });

    Ok(())
}
