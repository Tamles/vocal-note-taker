//! Audio module - handles audio capture and buffering
//!
//! Submodules:
//! - capture: cpal integration for microphone access
//! - buffer: WAV file writing

pub mod buffer;
pub mod capture;

// Re-exports for convenience
pub use buffer::{get_wav_path, save_wav};
pub use capture::{start_recording, RecordingHandle, RecordingResult, DEFAULT_SAMPLE_RATE};
