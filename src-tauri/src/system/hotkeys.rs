//! Global hotkeys module - keyboard shortcuts
//!
//! Registers global keyboard shortcuts for recording toggle.
//! Uses tauri-plugin-global-shortcut 2.x.
//!
//! Note: toggle_recording delegates to commands::start_recording and
//! commands::stop_recording to avoid code duplication (DRY principle).

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::commands::{self, AudioState};
use crate::error::AppError;

/// Default shortcut for toggle recording
/// CmdOrCtrl = Cmd on macOS, Ctrl on Linux/Windows
const DEFAULT_TOGGLE_RECORDING: &str = "CmdOrCtrl+Alt+R";

/// Registers global keyboard shortcuts for the application.
///
/// # Arguments
/// * `app` - The Tauri application handle
///
/// # Errors
/// Returns `AppError::HotkeyRegistrationFailed` if registration fails.
/// This is not fatal - app continues without global shortcuts.
pub fn register_global_shortcuts(app: &AppHandle) -> Result<(), AppError> {
    let shortcut: Shortcut = DEFAULT_TOGGLE_RECORDING
        .parse()
        .map_err(|e| AppError::HotkeyRegistrationFailed(format!("Invalid shortcut format: {}", e)))?;

    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            // Only trigger on key press, not release
            if event.state != ShortcutState::Pressed {
                return;
            }

            let app = app_handle.clone();

            // Execute toggle in async runtime
            tauri::async_runtime::spawn(async move {
                if let Err(e) = toggle_recording(&app).await {
                    eprintln!("Hotkey toggle recording error: {:?}", e);
                    // Emit error event to frontend (log if emit fails)
                    if let Err(emit_err) = app.emit(
                        "error",
                        serde_json::json!({
                            "type": "HotkeyError",
                            "message": format!("Erreur raccourci: {}", e)
                        }),
                    ) {
                        eprintln!("Failed to emit hotkey error event: {:?}", emit_err);
                    }
                }
            });
        })
        .map_err(|e| AppError::HotkeyRegistrationFailed(e.to_string()))?;

    // Register the shortcut
    app.global_shortcut()
        .register(shortcut)
        .map_err(|e| AppError::HotkeyRegistrationFailed(e.to_string()))?;

    println!("Global shortcut registered: {}", DEFAULT_TOGGLE_RECORDING);

    Ok(())
}

/// Toggle recording state: start if idle, stop if recording.
///
/// Delegates to commands::start_recording and commands::stop_recording
/// to avoid code duplication (DRY principle). The commands handle all
/// locking, event emission, and waveform channel setup.
async fn toggle_recording(app: &AppHandle) -> Result<(), AppError> {
    let state: tauri::State<'_, AudioState> = app.state();

    // Single lock acquisition to check AND decide action atomically (fixes TOCTOU)
    let is_recording = {
        let guard = state
            .recording
            .lock()
            .map_err(|_| AppError::RecordingInterrupted)?;
        guard.is_some()
    };

    if is_recording {
        // Delegate to stop_recording command
        match commands::stop_recording(app.state(), app.clone()).await {
            Ok(wav_path) => {
                println!("Recording stopped via hotkey: {}", wav_path);
            }
            Err(e) => {
                eprintln!("Failed to stop recording via hotkey: {:?}", e);
                return Err(e);
            }
        }
    } else {
        // Delegate to start_recording command
        match commands::start_recording(app.state(), app.clone()) {
            Ok(()) => {
                println!("Recording started via hotkey");
            }
            Err(e) => {
                eprintln!("Failed to start recording via hotkey: {:?}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Unregisters all global shortcuts.
/// Called during graceful shutdown.
pub fn unregister_all(app: &AppHandle) {
    if let Err(e) = app.global_shortcut().unregister_all() {
        eprintln!("Failed to unregister shortcuts: {:?}", e);
    } else {
        println!("Global shortcuts unregistered");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shortcut_constant() {
        // Verify the constant is correctly defined
        assert_eq!(DEFAULT_TOGGLE_RECORDING, "CmdOrCtrl+Alt+R");
    }

    #[test]
    fn test_shortcut_parse() {
        // Verify the shortcut string can be parsed
        let result: Result<Shortcut, _> = DEFAULT_TOGGLE_RECORDING.parse();
        assert!(result.is_ok(), "Shortcut should parse successfully");
    }
}
