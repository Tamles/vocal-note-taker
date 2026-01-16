// Module declarations
mod audio;
mod commands;
mod config;
mod error;
mod system;
mod transcription;

// Re-exports for external use
pub use error::AppError;

/// TODO: Remove after Story 1.2 validation - kept for manual testing
/// Temporary test command to verify error serialization via IPC.
/// This command is used during development to test error propagation
/// from backend to frontend.
#[tauri::command]
fn test_error(error_type: String) -> Result<String, AppError> {
    match error_type.as_str() {
        "microphone_denied" => Err(AppError::MicrophoneAccessDenied),
        "microphone_not_found" => Err(AppError::MicrophoneNotFound),
        "transcription" => Err(AppError::TranscriptionFailed("Test error".to_string())),
        "recording" => Err(AppError::RecordingInterrupted),
        "config" => Err(AppError::ConfigurationError("Test config error".to_string())),
        "clipboard" => Err(AppError::ClipboardError),
        _ => Ok("No error triggered".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_error])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
