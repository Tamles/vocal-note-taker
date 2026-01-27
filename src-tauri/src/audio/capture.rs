//! Audio capture module - cpal integration
//!
//! Gère la capture audio depuis le microphone système via cpal.
//! Supporte ALSA et PulseAudio sur Linux.
//!
//! Architecture: Le stream cpal tourne dans un thread dédié car Stream n'est pas Send/Sync.
//! Communication via channels tokio pour contrôle et données.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, StreamConfig};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::error::AppError;

/// Configuration audio par défaut (optimisée pour whisper.cpp)
pub const DEFAULT_SAMPLE_RATE: u32 = 16000;
pub const DEFAULT_CHANNELS: u16 = 1;

/// Ratio de downsampling pour waveform (1 sample sur 100)
const WAVEFORM_DOWNSAMPLE_RATIO: usize = 100;

/// Handle vers le système d'enregistrement audio
/// Ce handle est Send + Sync car il communique via channels
pub struct RecordingHandle {
    /// Channel pour demander l'arrêt de l'enregistrement
    stop_tx: Option<oneshot::Sender<()>>,
    /// Receiver pour obtenir les samples à la fin
    samples_rx: Option<oneshot::Receiver<Vec<f32>>>,
    /// Sample rate réel utilisé pour la capture (peut différer de DEFAULT_SAMPLE_RATE)
    pub sample_rate: u32,
}

/// Résultat de l'arrêt d'un enregistrement
pub struct RecordingResult {
    /// Samples audio capturés
    pub samples: Vec<f32>,
    /// Sample rate réel utilisé pendant la capture
    pub sample_rate: u32,
}

impl RecordingHandle {
    /// Arrête l'enregistrement et retourne les samples capturés avec le sample rate
    pub async fn stop(mut self) -> Result<RecordingResult, AppError> {
        // Envoyer le signal d'arrêt
        if let Some(stop_tx) = self.stop_tx.take() {
            let _ = stop_tx.send(());
        }

        // Attendre les samples
        if let Some(samples_rx) = self.samples_rx.take() {
            let samples = samples_rx
                .await
                .map_err(|_| AppError::RecordingInterrupted)?;
            Ok(RecordingResult {
                samples,
                sample_rate: self.sample_rate,
            })
        } else {
            Err(AppError::RecordingInterrupted)
        }
    }
}

/// Obtient le device d'entrée audio par défaut
pub fn get_default_input_device() -> Result<Device, AppError> {
    let host = cpal::default_host();

    host.default_input_device()
        .ok_or(AppError::MicrophoneNotFound)
}

/// Trouve une configuration supportée proche de 16kHz mono
fn get_supported_config(device: &Device) -> Result<StreamConfig, AppError> {
    let supported_configs = device
        .supported_input_configs()
        .map_err(|_| AppError::MicrophoneAccessDenied)?;

    // Chercher une config qui supporte 16kHz
    for config_range in supported_configs {
        if config_range.min_sample_rate().0 <= DEFAULT_SAMPLE_RATE
            && config_range.max_sample_rate().0 >= DEFAULT_SAMPLE_RATE
            && config_range.channels() >= DEFAULT_CHANNELS
        {
            return Ok(StreamConfig {
                channels: DEFAULT_CHANNELS,
                sample_rate: cpal::SampleRate(DEFAULT_SAMPLE_RATE),
                buffer_size: cpal::BufferSize::Default,
            });
        }
    }

    // Fallback: utiliser la config par défaut du device
    let default_config = device
        .default_input_config()
        .map_err(|_| AppError::MicrophoneAccessDenied)?;

    Ok(StreamConfig {
        channels: DEFAULT_CHANNELS,
        sample_rate: default_config.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    })
}

/// Démarre l'enregistrement audio dans un thread dédié
///
/// # Arguments
/// * `waveform_tx` - Channel pour envoyer samples vers frontend (waveform display)
///
/// # Returns
/// RecordingHandle pour contrôler et arrêter l'enregistrement
///
/// # Errors
/// - `MicrophoneNotFound` si aucun microphone détecté
/// - `MicrophoneAccessDenied` si permissions insuffisantes
pub fn start_recording(
    waveform_tx: Option<Sender<Vec<f32>>>,
) -> Result<RecordingHandle, AppError> {
    // Vérifier que le device est disponible AVANT de spawner le thread
    let device = get_default_input_device()?;
    let config = get_supported_config(&device)?;

    // Capturer le sample rate réel AVANT de déplacer config dans le thread
    let actual_sample_rate = config.sample_rate.0;

    // Channels pour contrôle
    let (stop_tx, stop_rx) = oneshot::channel::<()>();
    let (samples_tx, samples_rx) = oneshot::channel::<Vec<f32>>();
    let (init_tx, init_rx) = std::sync::mpsc::channel::<Result<(), AppError>>();

    // Spawner le thread audio
    thread::spawn(move || {
        run_audio_thread(device, config, waveform_tx, stop_rx, samples_tx, init_tx);
    });

    // Attendre l'initialisation du stream
    match init_rx.recv() {
        Ok(Ok(())) => Ok(RecordingHandle {
            stop_tx: Some(stop_tx),
            samples_rx: Some(samples_rx),
            sample_rate: actual_sample_rate,
        }),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(AppError::RecordingInterrupted),
    }
}

/// Thread principal pour la capture audio
fn run_audio_thread(
    device: Device,
    config: StreamConfig,
    waveform_tx: Option<Sender<Vec<f32>>>,
    stop_rx: oneshot::Receiver<()>,
    samples_tx: oneshot::Sender<Vec<f32>>,
    init_tx: std::sync::mpsc::Sender<Result<(), AppError>>,
) {
    let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = buffer.clone();
    let sample_counter = Arc::new(Mutex::new(0usize));

    // Construire le stream
    let stream_result = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Écrire tous les samples dans le buffer WAV
            if let Ok(mut buf) = buffer_clone.lock() {
                buf.extend_from_slice(data);
            }

            // Envoyer samples downsampleés pour waveform (si channel fourni)
            if let Some(ref tx) = waveform_tx {
                let mut counter = sample_counter
                    .lock()
                    .expect("Counter lock poisoned - should never happen");
                let waveform_samples: Vec<f32> = data
                    .iter()
                    .filter_map(|&sample| {
                        *counter += 1;
                        if *counter % WAVEFORM_DOWNSAMPLE_RATIO == 0 {
                            Some(sample)
                        } else {
                            None
                        }
                    })
                    .collect();

                if !waveform_samples.is_empty() {
                    let _ = tx.try_send(waveform_samples);
                }
            }
        },
        move |err| {
            eprintln!("Audio stream error: {:?}", err);
        },
        None,
    );

    let stream = match stream_result {
        Ok(s) => s,
        Err(e) => {
            let err = match e {
                cpal::BuildStreamError::DeviceNotAvailable => AppError::MicrophoneNotFound,
                cpal::BuildStreamError::StreamConfigNotSupported => AppError::MicrophoneAccessDenied,
                _ => AppError::RecordingInterrupted,
            };
            let _ = init_tx.send(Err(err));
            return;
        }
    };

    // Démarrer le stream
    if let Err(_) = stream.play() {
        let _ = init_tx.send(Err(AppError::RecordingInterrupted));
        return;
    }

    // Signaler que l'initialisation est réussie
    let _ = init_tx.send(Ok(()));

    // Attendre le signal d'arrêt (blocking)
    let _ = stop_rx.blocking_recv();

    // Arrêter le stream
    drop(stream);

    // Récupérer et envoyer les samples
    // unwrap_or_default: si le mutex est poisonné (panic dans callback), retourner Vec vide
    // plutôt que propager le panic - l'utilisateur recevra un fichier audio vide mais l'app survit
    let samples = buffer
        .lock()
        .map(|b| b.clone())
        .unwrap_or_default();
    let _ = samples_tx.send(samples);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_SAMPLE_RATE, 16000);
        assert_eq!(DEFAULT_CHANNELS, 1);
    }

    #[test]
    fn test_get_default_input_device() {
        let result = get_default_input_device();
        // Result peut être Ok ou Err selon l'environnement
        assert!(result.is_ok() || matches!(result, Err(AppError::MicrophoneNotFound)));
    }

    #[test]
    fn test_start_recording_returns_correct_error_without_mic() {
        let result = start_recording(None);

        match result {
            Ok(_) => {
                // Microphone disponible - test passé
            }
            Err(AppError::MicrophoneNotFound) => {
                // Pas de microphone - comportement attendu
            }
            Err(AppError::MicrophoneAccessDenied) => {
                // Permission refusée - comportement attendu
            }
            Err(e) => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
    }
}
