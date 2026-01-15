use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Microphone access denied")]
    MicrophoneAccessDenied,

    #[error("Microphone not found")]
    MicrophoneNotFound,

    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    #[error("Recording interrupted")]
    RecordingInterrupted,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Clipboard error")]
    ClipboardError,
}
