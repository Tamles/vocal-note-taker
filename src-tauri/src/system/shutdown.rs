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
/// # Returns
/// Returns `Ok(())` on success. Logs the number of orphaned files removed if any.
///
/// # Errors
/// Returns `AppError::IoError` if unable to read the temp directory.
/// Individual file deletion failures are logged but don't cause the function to fail.
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

    let mut removed_count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "wav") {
            if let Err(e) = fs::remove_file(&path) {
                // Log but don't fail - best effort cleanup
                eprintln!("Warning: Could not remove temp file {:?}: {}", path, e);
            } else {
                removed_count += 1;
            }
        }
    }

    // Log number of orphaned files removed (if any) - useful for detecting crash recovery
    if removed_count > 0 {
        println!(
            "NFR-SEC-3: Removed {} orphaned audio file(s) from temp directory",
            removed_count
        );
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
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Returns an isolated test directory to avoid conflicts with real app data.
    /// Each test gets a unique subdirectory based on timestamp and thread ID.
    fn get_isolated_test_dir(test_name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let thread_id = format!("{:?}", std::thread::current().id())
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        std::env::temp_dir()
            .join("vocal-note-taker-tests")
            .join(format!("{}_{}_{}",test_name, thread_id, timestamp))
    }

    /// Cleanup helper that works on a specific directory (for test isolation).
    fn cleanup_test_dir(temp_dir: &PathBuf) -> Result<(), AppError> {
        if !temp_dir.exists() {
            return Ok(());
        }

        let entries = fs::read_dir(temp_dir).map_err(|e| {
            AppError::IoError(format!("Cannot read temp directory: {}", e))
        })?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "wav") {
                let _ = fs::remove_file(&path);
            }
        }

        Ok(())
    }

    /// Ensures test directory is cleaned up after test (RAII pattern).
    struct TestDirGuard {
        path: PathBuf,
    }

    impl Drop for TestDirGuard {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

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
        // Use isolated test directory
        let test_dir = get_isolated_test_dir("cleanup_removes_wav");
        let _guard = TestDirGuard { path: test_dir.clone() };

        if let Err(e) = fs::create_dir_all(&test_dir) {
            eprintln!("Could not create test dir: {}", e);
            return;
        }

        // Create a test .wav file
        let test_file = test_dir.join("test_cleanup.wav");
        if File::create(&test_file).is_ok() {
            assert!(test_file.exists());

            // Run cleanup on isolated dir
            let result = cleanup_test_dir(&test_dir);
            assert!(result.is_ok());

            // File should be removed
            assert!(!test_file.exists());
        }
    }

    #[test]
    fn test_temp_dir_empty_after_cleanup() {
        // AC #4: temp/ vide après toute opération
        // Use isolated test directory
        let test_dir = get_isolated_test_dir("temp_dir_empty");
        let _guard = TestDirGuard { path: test_dir.clone() };

        if let Err(e) = fs::create_dir_all(&test_dir) {
            eprintln!("Could not create test dir: {}", e);
            return;
        }

        // Create multiple .wav files to simulate orphaned files
        let files = vec![
            test_dir.join("orphan1.wav"),
            test_dir.join("orphan2.wav"),
            test_dir.join("recording.wav"),
        ];

        for f in &files {
            if let Err(e) = File::create(f) {
                eprintln!("Could not create test file {:?}: {}", f, e);
                return;
            }
        }

        // Verify files exist
        for f in &files {
            assert!(f.exists(), "Test file should exist: {:?}", f);
        }

        // Run cleanup on isolated dir
        let result = cleanup_test_dir(&test_dir);
        assert!(result.is_ok(), "cleanup should succeed");

        // Verify NO .wav files remain
        let remaining_wav_files: Vec<_> = fs::read_dir(&test_dir)
            .unwrap()
            .flatten()
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "wav"))
            .collect();

        assert!(
            remaining_wav_files.is_empty(),
            "test dir should have no .wav files after cleanup, found: {:?}",
            remaining_wav_files
                .iter()
                .map(|e| e.path())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_cleanup_preserves_non_wav_files() {
        // Ensure cleanup only removes .wav files, not other data
        // Use isolated test directory
        let test_dir = get_isolated_test_dir("preserves_non_wav");
        let _guard = TestDirGuard { path: test_dir.clone() };

        if let Err(e) = fs::create_dir_all(&test_dir) {
            eprintln!("Could not create test dir: {}", e);
            return;
        }

        let wav_file = test_dir.join("test.wav");
        let txt_file = test_dir.join("test.txt");

        // Create both files
        File::create(&wav_file).ok();
        File::create(&txt_file).ok();

        // Run cleanup on isolated dir
        let _ = cleanup_test_dir(&test_dir);

        // .wav should be removed, .txt should remain
        assert!(!wav_file.exists(), ".wav file should be removed");
        assert!(txt_file.exists(), ".txt file should be preserved");
        // txt_file is cleaned up by TestDirGuard
    }

    #[test]
    fn test_startup_cleanup_removes_orphans() {
        // AC #3: Fichiers orphelins supprimés au prochain démarrage
        // Use isolated test directory
        let test_dir = get_isolated_test_dir("startup_cleanup");
        let _guard = TestDirGuard { path: test_dir.clone() };

        if let Err(e) = fs::create_dir_all(&test_dir) {
            eprintln!("Could not create test dir: {}", e);
            return;
        }

        // Create orphaned files (simulating crash)
        let orphan_files = vec![
            test_dir.join("recording.wav"),
            test_dir.join("old_recording.wav"),
        ];

        for f in &orphan_files {
            File::create(f).ok();
        }

        // Verify orphans exist
        for f in &orphan_files {
            assert!(f.exists(), "Orphan file should exist before cleanup: {:?}", f);
        }

        // Simulate startup cleanup on isolated dir
        let result = cleanup_test_dir(&test_dir);
        assert!(result.is_ok(), "Startup cleanup should succeed");

        // Verify all orphans are removed
        for f in &orphan_files {
            assert!(!f.exists(), "Orphan file should be removed: {:?}", f);
        }
    }

    #[test]
    fn test_cleanup_handles_empty_temp_dir() {
        // Edge case: temp dir exists but is empty
        // Use isolated test directory
        let test_dir = get_isolated_test_dir("empty_temp_dir");
        let _guard = TestDirGuard { path: test_dir.clone() };

        if let Err(e) = fs::create_dir_all(&test_dir) {
            eprintln!("Could not create test dir: {}", e);
            return;
        }

        // Cleanup should succeed on empty dir
        let result = cleanup_test_dir(&test_dir);
        assert!(result.is_ok(), "Cleanup should succeed on empty temp dir");
    }

    #[test]
    fn test_cleanup_idempotent() {
        // Running cleanup multiple times should be safe
        // Use isolated test directory
        let test_dir = get_isolated_test_dir("idempotent");
        let _guard = TestDirGuard { path: test_dir.clone() };

        if let Err(e) = fs::create_dir_all(&test_dir) {
            eprintln!("Could not create test dir: {}", e);
            return;
        }

        // Create a file
        let wav_file = test_dir.join("idempotent_test.wav");
        File::create(&wav_file).ok();

        // First cleanup
        let result1 = cleanup_test_dir(&test_dir);
        assert!(result1.is_ok());
        assert!(!wav_file.exists());

        // Second cleanup should still succeed (no files to remove)
        let result2 = cleanup_test_dir(&test_dir);
        assert!(result2.is_ok());

        // Third cleanup
        let result3 = cleanup_test_dir(&test_dir);
        assert!(result3.is_ok());
    }

    #[test]
    fn test_production_cleanup_function() {
        // Test the actual production cleanup_temp_files() function
        // This ensures the real function works correctly
        let temp_dir = get_temp_dir();

        // Create temp directory if it doesn't exist
        if fs::create_dir_all(&temp_dir).is_err() {
            return; // Skip test if we can't create the dir
        }

        // Create a test file with unique name to avoid conflicts
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_file = temp_dir.join(format!("prod_test_{}.wav", timestamp));

        if File::create(&test_file).is_ok() {
            assert!(test_file.exists());

            // Run production cleanup
            let result = cleanup_temp_files();
            assert!(result.is_ok());

            // File should be removed
            assert!(!test_file.exists());
        }
    }
}
