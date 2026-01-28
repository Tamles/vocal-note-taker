// Module declarations
mod audio;
mod commands;
mod config;
mod error;
mod system;
mod transcription;

use tauri::menu::{Menu, MenuItem};
use tauri::Manager;

use crate::commands::AudioState;
use crate::system::hotkeys;
use crate::transcription::WhisperState;

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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AudioState::default())
        .manage(WhisperState::default())
        .invoke_handler(tauri::generate_handler![
            test_error,
            commands::get_version,
            commands::request_quit,
            commands::start_recording,
            commands::stop_recording,
            commands::start_transcription
        ])
        .setup(|app| {
            // Create application menu with Quit item (Ctrl+Q)
            let quit_item = MenuItem::with_id(
                app,
                "quit",
                "Quitter",
                true,
                Some("CmdOrCtrl+Q"),
            )?;

            let menu = Menu::with_items(app, &[&quit_item])?;
            app.set_menu(menu)?;

            // Register global shortcuts (Story 2.5)
            // Non-fatal: app continues without shortcuts if registration fails
            if let Err(e) = hotkeys::register_global_shortcuts(&app.handle()) {
                eprintln!("Warning: Could not register global shortcuts: {:?}", e);
                eprintln!("Recording via button still available.");
            }

            // Check Whisper model availability (Story 3.1)
            // Non-fatal: app continues without model, transcription unavailable until installed
            match crate::transcription::check_model_availability() {
                Ok(path) => {
                    println!("Whisper model found at: {}", path.display());
                }
                Err(e) => {
                    eprintln!("Warning: {}", e);
                    eprintln!("Transcription will not be available until model is installed.");
                }
            }

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id() == "quit" {
                // Clone app handle for async task
                let app_handle = app.clone();

                // Unregister global shortcuts before exit
                hotkeys::unregister_all(&app_handle);

                // Perform graceful shutdown asynchronously
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = crate::system::shutdown::graceful_shutdown() {
                        eprintln!("Error during shutdown cleanup: {:?}", e);
                    }
                    // Exit application after cleanup
                    app_handle.exit(0);
                });
            }
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Prevent default close behavior
                api.prevent_close();

                // Clone app handle for async task
                let app = window.app_handle().clone();

                // Unregister global shortcuts before exit
                hotkeys::unregister_all(&app);

                // Perform graceful shutdown asynchronously
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = crate::system::shutdown::graceful_shutdown() {
                        eprintln!("Error during shutdown cleanup: {:?}", e);
                    }
                    // Exit application after cleanup
                    app.exit(0);
                });
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
