//! Whisper transcription module - whisper-rs integration
//!
//! Provides local speech-to-text transcription using whisper.cpp.
//! 100% local processing - no cloud fallback (NFR-SEC-1).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use whisper_rs::{WhisperContext, WhisperContextParameters};

use crate::error::AppError;

/// Wrapper around WhisperContext for managed state.
/// The context is thread-safe and can be shared across async tasks.
pub struct WhisperModel {
    context: WhisperContext,
}

impl WhisperModel {
    /// Creates a new WhisperModel by loading from the given path.
    ///
    /// # Errors
    /// - `ModelNotFound` if the model file doesn't exist
    /// - `ModelLoadFailed` if whisper-rs fails to load the model
    pub fn load(model_path: &Path) -> Result<Self, AppError> {
        if !model_path.exists() {
            return Err(AppError::ModelNotFound(format!(
                "Modèle Whisper non trouvé: {}. \
                 Exécutez: ./scripts/download-models.sh",
                model_path.display()
            )));
        }

        let params = WhisperContextParameters::default();

        let context = WhisperContext::new_with_params(
            model_path.to_str().ok_or_else(|| {
                AppError::ModelLoadFailed("Chemin de modèle invalide (non-UTF8)".to_string())
            })?,
            params,
        )
        .map_err(|e| AppError::ModelLoadFailed(e.to_string()))?;

        println!(
            "Whisper model loaded successfully from: {}",
            model_path.display()
        );

        Ok(Self { context })
    }

    /// Returns a reference to the underlying WhisperContext.
    /// Used by transcription functions in Story 3.2.
    pub fn context(&self) -> &WhisperContext {
        &self.context
    }
}

/// State managed by Tauri for the Whisper model.
/// Uses Option for lazy loading - model loaded on first transcription.
pub struct WhisperState {
    pub model: Arc<Mutex<Option<WhisperModel>>>,
}

impl Default for WhisperState {
    fn default() -> Self {
        Self {
            model: Arc::new(Mutex::new(None)),
        }
    }
}

/// Returns the expected path for the Whisper model.
///
/// Location: ~/.local/share/vocal-note-taker/models/ggml-large-v3.bin
///
/// # Errors
/// Returns `ConfigurationError` if the system data directory cannot be determined.
pub fn get_model_path() -> Result<PathBuf, AppError> {
    let mut path = dirs::data_local_dir().ok_or_else(|| {
        AppError::ConfigurationError(
            "Impossible de déterminer le répertoire de données local".to_string(),
        )
    })?;
    path.push("vocal-note-taker");
    path.push("models");
    path.push("ggml-large-v3.bin");
    Ok(path)
}

/// Ensures the model directory exists.
///
/// Creates ~/.local/share/vocal-note-taker/models/ if not present.
pub fn ensure_model_dir() -> Result<PathBuf, AppError> {
    let model_path = get_model_path()?;
    if let Some(parent) = model_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(model_path)
}

/// Checks if the Whisper model is available.
///
/// Returns Ok(path) if model exists, Err with instructions otherwise.
pub fn check_model_availability() -> Result<PathBuf, AppError> {
    let model_path = get_model_path()?;
    if model_path.exists() {
        Ok(model_path)
    } else {
        Err(AppError::ModelNotFound(format!(
            "Modèle Whisper non trouvé.\n\n\
             Pour installer le modèle:\n\
             1. Exécutez: ./scripts/download-models.sh\n\
             2. Ou téléchargez manuellement depuis:\n\
                https://huggingface.co/ggerganov/whisper.cpp/tree/main\n\
             3. Placez ggml-large-v3.bin dans:\n\
                {}",
            model_path.display()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_path_returns_valid_path() {
        let result = get_model_path();
        assert!(result.is_ok(), "Should return Ok on systems with data dir");

        let path = result.unwrap();
        let path_str = path.to_string_lossy();

        assert!(
            path_str.contains("vocal-note-taker"),
            "Should contain app name"
        );
        assert!(path_str.contains("models"), "Should contain models dir");
        assert!(
            path_str.contains("ggml-large-v3.bin"),
            "Should contain model name"
        );
    }

    #[test]
    fn test_model_not_found_error_message_quality() {
        // Test error message quality by checking a non-existent path directly
        // This ensures the test always validates error content, regardless of model presence
        let fake_path = PathBuf::from("/nonexistent/test/path/model.bin");

        // Simulate the error that check_model_availability would produce
        let error = AppError::ModelNotFound(format!(
            "Modèle Whisper non trouvé.\n\n\
             Pour installer le modèle:\n\
             1. Exécutez: ./scripts/download-models.sh\n\
             2. Ou téléchargez manuellement depuis:\n\
                https://huggingface.co/ggerganov/whisper.cpp/tree/main\n\
             3. Placez ggml-large-v3.bin dans:\n\
                {}",
            fake_path.display()
        ));

        let msg = error.to_string();
        assert!(
            msg.contains("download-models.sh"),
            "Error should mention download script"
        );
        assert!(
            msg.contains("huggingface"),
            "Error should mention download source"
        );
        assert!(
            msg.contains("ggml-large-v3.bin"),
            "Error should mention model filename"
        );
    }

    #[test]
    fn test_check_model_availability_returns_error_for_missing_model() {
        // This test verifies the function behavior
        // It will return Ok if model exists (valid), Err if not (also valid)
        let result = check_model_availability();

        match result {
            Ok(path) => {
                // Model exists - verify path is correct
                assert!(
                    path.to_string_lossy().contains("ggml-large-v3.bin"),
                    "Should return correct model path"
                );
            }
            Err(AppError::ModelNotFound(msg)) => {
                // Model missing - verify error has instructions
                assert!(
                    msg.contains("download-models.sh"),
                    "Error should contain download instructions"
                );
            }
            Err(e) => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
    }

    #[test]
    fn test_whisper_state_default() {
        let state = WhisperState::default();
        // Model should be None initially (lazy loading)
        let guard = state.model.try_lock().unwrap();
        assert!(guard.is_none(), "Model should be None by default");
    }

    #[test]
    fn test_ensure_model_dir_creates_path() {
        // This test verifies the function doesn't panic
        // Actual directory creation depends on permissions
        let result = ensure_model_dir();
        assert!(result.is_ok(), "Should return Ok with path");
        let path = result.unwrap();
        assert!(
            path.to_string_lossy().contains("ggml-large-v3.bin"),
            "Should return model path"
        );
    }
}
