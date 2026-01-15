// Module declarations
mod audio;
mod commands;
mod config;
mod error;
mod system;
mod transcription;

// Re-exports for external use
pub use error::AppError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
