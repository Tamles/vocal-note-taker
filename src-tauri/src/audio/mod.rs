//! Audio module - handles audio capture and buffering
//!
//! Submodules:
//! - capture: cpal integration for microphone access
//! - buffer: double buffer logic for recording

pub mod buffer;
pub mod capture;
