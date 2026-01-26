//! Tauri commands (IPC layer) - THIN orchestration
//!
//! This module contains all Tauri commands that serve as the interface
//! between the frontend and backend. Commands should be thin wrappers
//! that delegate to domain modules.

use tauri::AppHandle;

use crate::error::AppError;
use crate::system::shutdown;

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

// Future Tauri commands will be added as stories are implemented:
// - start_recording() -> Result<(), AppError>
// - stop_recording() -> Result<String, AppError>
// - load_config() -> Result<AppConfig, AppError>
// - copy_to_clipboard(text: String) -> Result<(), AppError>
