//! Shutdown module - graceful application shutdown
//!
//! Handles cleanup of temporary files and graceful shutdown sequence.
//! Prepares for future Ghost Mode (Epic 5) where close != quit.

use std::fs;
use std::path::PathBuf;

use crate::error::AppError;

/// Returns the path to the application's temporary directory.
///
/// Platform paths:
/// - Linux: ~/.local/share/vocal-note-taker/temp/
/// - macOS: ~/Library/Application Support/vocal-note-taker/temp/
fn get_temp_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("vocal-note-taker")
        .join("temp")
}

/// Cleans up all temporary files (*.wav) from the temp directory.
///
/// # Errors
/// Returns `AppError::IoError` if unable to read or delete files.
/// Silently succeeds if the temp directory doesn't exist.
pub fn cleanup_temp_files() -> Result<(), AppError> {
    let temp_dir = get_temp_dir();

    if !temp_dir.exists() {
        // No temp directory, nothing to clean
        return Ok(());
    }

    let entries = fs::read_dir(&temp_dir).map_err(|e| {
        AppError::IoError(format!("Cannot read temp directory: {}", e))
    })?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "wav") {
            if let Err(e) = fs::remove_file(&path) {
                // Log but don't fail - best effort cleanup
                eprintln!("Warning: Could not remove temp file {:?}: {}", path, e);
            }
        }
    }

    Ok(())
}

/// Performs a graceful shutdown of the application.
///
/// This function:
/// 1. Stops any active recording (TODO: Story 2.x)
/// 2. Cleans up temporary files
///
/// # Errors
/// Returns `AppError` if cleanup fails critically.
pub fn graceful_shutdown() -> Result<(), AppError> {
    // TODO (Story 2.x): Check and stop active recording
    // For now, just cleanup temporary files

    cleanup_temp_files()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_cleanup_temp_files_no_dir() {
        // Should succeed even if temp dir doesn't exist
        let result = cleanup_temp_files();
        assert!(result.is_ok());
    }

    #[test]
    fn test_graceful_shutdown() {
        let result = graceful_shutdown();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_temp_dir_returns_valid_path() {
        let path = get_temp_dir();
        // Should contain vocal-note-taker/temp
        assert!(path.to_string_lossy().contains("vocal-note-taker"));
        assert!(path.to_string_lossy().ends_with("temp"));
    }

    #[test]
    fn test_cleanup_removes_wav_files() {
        let temp_dir = get_temp_dir();

        // Create temp directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&temp_dir) {
            eprintln!("Could not create temp dir for test: {}", e);
            return;
        }

        // Create a test .wav file
        let test_file = temp_dir.join("test_cleanup.wav");
        if File::create(&test_file).is_ok() {
            assert!(test_file.exists());

            // Run cleanup
            let result = cleanup_temp_files();
            assert!(result.is_ok());

            // File should be removed
            assert!(!test_file.exists());
        }
    }
}
