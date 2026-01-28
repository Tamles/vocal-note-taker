//! Transcription module - whisper-rs integration
//!
//! Submodules:
//! - whisper: whisper-rs binding for local transcription
//!
//! 100% local processing - no cloud fallback (NFR-SEC-1).

pub mod whisper;

// Re-exports for convenient access
pub use whisper::{
    check_model_availability, ensure_model_dir, get_model_path, transcribe_audio, WhisperModel,
    WhisperState,
};
