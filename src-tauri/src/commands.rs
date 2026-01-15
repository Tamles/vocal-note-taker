//! Tauri commands (IPC layer) - THIN orchestration
//!
//! This module contains all Tauri commands that serve as the interface
//! between the frontend and backend. Commands should be thin wrappers
//! that delegate to domain modules.

use crate::error::AppError;

// Placeholder for future Tauri commands
// Commands will be added as stories are implemented:
// - start_recording() -> Result<(), AppError>
// - stop_recording() -> Result<String, AppError>
// - load_config() -> Result<AppConfig, AppError>
// - copy_to_clipboard(text: String) -> Result<(), AppError>
