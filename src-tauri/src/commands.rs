//! Tauri commands (IPC layer) - THIN orchestration
//!
//! This module contains all Tauri commands that serve as the interface
//! between the frontend and backend. Commands should be thin wrappers
//! that delegate to domain modules.

use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::mpsc;

use crate::audio::{self, RecordingHandle};
use crate::error::AppError;
use crate::system::shutdown;

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
