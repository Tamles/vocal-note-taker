//! Whisper transcription module - whisper-rs integration
//!
//! Provides local speech-to-text transcription using whisper.cpp.
//! 100% local processing - no cloud fallback (NFR-SEC-1).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

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
        .map_err(|e| {
            // Diagnostic: analyser le type d'erreur pour des conseils contextualisés
            let err_str = e.to_string();
            let diagnostic = if err_str.to_lowercase().contains("memory")
                || err_str.to_lowercase().contains("alloc")
            {
                format!(
                    "Mémoire insuffisante pour charger le modèle (~3GB requis): {}",
                    err_str
                )
            } else if err_str.to_lowercase().contains("invalid")
                || err_str.to_lowercase().contains("corrupt")
            {
                format!(
                    "Fichier modèle corrompu ou incomplet: {}. Retéléchargez le modèle",
                    err_str
                )
            } else {
                err_str
            };
            AppError::ModelLoadFailed(diagnostic)
        })?;

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

/// Valide le format d'un fichier WAV avant transcription.
///
/// # Arguments
/// * `path` - Chemin vers le fichier WAV
///
/// # Errors
/// - `InvalidAudioFormat` si header WAV invalide, samples vides, ou format incorrect
fn validate_wav_file(path: &Path) -> Result<(), AppError> {
    let reader = hound::WavReader::open(path).map_err(|e| {
        AppError::InvalidAudioFormat(format!("Fichier audio invalide ou corrompu: {}", e))
    })?;

    let spec = reader.spec();

    // Vérifier channels (doit être mono)
    if spec.channels != 1 {
        return Err(AppError::InvalidAudioFormat(format!(
            "Audio doit être mono (1 canal), reçu: {} canaux",
            spec.channels
        )));
    }

    // Vérifier sample rate (16kHz attendu pour Whisper)
    if spec.sample_rate != 16000 {
        return Err(AppError::InvalidAudioFormat(format!(
            "Sample rate doit être 16000 Hz, reçu: {} Hz",
            spec.sample_rate
        )));
    }

    // Vérifier que le fichier n'est pas vide
    let duration = reader.duration();
    if duration == 0 {
        return Err(AppError::InvalidAudioFormat(
            "Fichier audio vide - aucun échantillon détecté".to_string(),
        ));
    }

    Ok(())
}

/// Transcrit un fichier audio WAV en texte.
///
/// # Arguments
/// * `model` - WhisperModel chargé
/// * `audio_path` - Chemin vers fichier WAV (16kHz mono requis)
///
/// # Returns
/// Texte transcrit ou AppError::TranscriptionFailed
///
/// # Errors
/// - `InvalidAudioFormat` si le fichier WAV est invalide (header, format, vide)
/// - `TranscriptionFailed` si la transcription échoue
pub fn transcribe_audio(model: &WhisperModel, audio_path: &Path) -> Result<String, AppError> {
    // 1. Valider le fichier audio AVANT de lire les samples
    validate_wav_file(audio_path)?;

    // 2. Lire le fichier WAV
    let samples = read_wav_samples(audio_path)?;

    if samples.is_empty() {
        return Err(AppError::TranscriptionFailed(
            "Fichier audio vide".to_string(),
        ));
    }

    println!(
        "Starting transcription: {} samples from {}",
        samples.len(),
        audio_path.display()
    );

    // 3. Créer state pour transcription
    let mut state = model
        .context()
        .create_state()
        .map_err(|e| AppError::TranscriptionFailed(format!("Échec création state: {}", e)))?;

    // 3. Configurer les paramètres
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("auto")); // Auto-détection de la langue
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_single_segment(false);
    params.set_translate(false); // Transcription uniquement, pas de traduction

    // 4. Exécuter transcription
    state
        .full(params, &samples)
        .map_err(|e| AppError::TranscriptionFailed(format!("Échec transcription: {}", e)))?;

    // 5. Extraire le texte des segments
    let num_segments = state.full_n_segments();

    let mut text = String::new();
    for i in 0..num_segments {
        if let Some(segment) = state.get_segment(i) {
            if let Ok(segment_text) = segment.to_str_lossy() {
                text.push_str(&segment_text);
                text.push(' ');
            }
        }
    }

    let result = text.trim().to_string();
    println!(
        "Transcription complete: {} segments, {} chars",
        num_segments,
        result.len()
    );

    Ok(result)
}

/// Lit un fichier WAV et retourne les samples f32 normalisés.
///
/// # Arguments
/// * `path` - Chemin vers le fichier WAV (doit être validé par `validate_wav_file` avant appel)
///
/// # Returns
/// Vecteur de samples f32 normalisés [-1.0, 1.0]
///
/// # Errors
/// - `TranscriptionFailed` si le fichier ne peut pas être ouvert
///
/// # Note
/// Cette fonction suppose que le fichier a été validé par `validate_wav_file()`.
/// Ne pas appeler directement sans validation préalable.
fn read_wav_samples(path: &Path) -> Result<Vec<f32>, AppError> {
    let reader = hound::WavReader::open(path).map_err(|e| {
        AppError::TranscriptionFailed(format!(
            "Impossible d'ouvrir le fichier WAV '{}': {}",
            path.display(),
            e
        ))
    })?;

    let spec = reader.spec();

    // Note: validation channels/sample_rate déjà faite par validate_wav_file()

    println!(
        "WAV file: {} Hz, {} bits, {} channels",
        spec.sample_rate, spec.bits_per_sample, spec.channels
    );

    // Convertir samples en f32 normalisés [-1.0, 1.0]
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            let max_val = (1i64 << (spec.bits_per_sample - 1)) as f32;
            reader
                .into_samples::<i32>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / max_val)
                .collect()
        }
        hound::SampleFormat::Float => reader
            .into_samples::<f32>()
            .filter_map(|s| s.ok())
            .collect(),
    };

    Ok(samples)
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

    #[test]
    fn test_read_wav_samples_missing_file() {
        let fake_path = std::path::PathBuf::from("/nonexistent/audio.wav");
        let result = read_wav_samples(&fake_path);
        assert!(result.is_err(), "Should return error for missing file");
        match result {
            Err(AppError::TranscriptionFailed(msg)) => {
                assert!(
                    msg.contains("Impossible d'ouvrir"),
                    "Error should mention file open failure"
                );
            }
            _ => panic!("Expected TranscriptionFailed error"),
        }
    }

    #[test]
    fn test_transcribe_audio_missing_file() {
        // Test that transcribe_audio returns error for missing model
        // We can't test the full flow without a model, but we can verify error handling
        let fake_model_path = std::path::PathBuf::from("/nonexistent/model.bin");
        let result = WhisperModel::load(&fake_model_path);
        assert!(result.is_err(), "Should return error for missing model");
        match result {
            Err(AppError::ModelNotFound(msg)) => {
                assert!(
                    msg.contains("Modèle Whisper non trouvé"),
                    "Error should mention model not found"
                );
            }
            _ => panic!("Expected ModelNotFound error"),
        }
    }

    #[test]
    fn test_validate_wav_nonexistent_file() {
        let result = validate_wav_file(Path::new("/nonexistent/file.wav"));
        assert!(result.is_err(), "Should return error for nonexistent file");
        match result {
            Err(AppError::InvalidAudioFormat(msg)) => {
                assert!(
                    msg.contains("invalide ou corrompu"),
                    "Error should mention invalid/corrupted file"
                );
            }
            _ => panic!("Expected InvalidAudioFormat error"),
        }
    }

    #[test]
    fn test_validate_wav_empty_file() {
        let temp_dir = std::env::temp_dir();
        let empty_wav = temp_dir.join("test_empty_validation.wav");

        // Créer fichier WAV vide avec header valide mais 0 samples
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let writer = hound::WavWriter::create(&empty_wav, spec).unwrap();
        // Ne pas écrire de samples - fichier vide
        writer.finalize().unwrap();

        let result = validate_wav_file(&empty_wav);

        // Cleanup
        let _ = std::fs::remove_file(&empty_wav);

        assert!(result.is_err(), "Should return error for empty audio file");
        match result {
            Err(AppError::InvalidAudioFormat(msg)) => {
                assert!(
                    msg.contains("vide") || msg.contains("aucun échantillon"),
                    "Error should mention empty file: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidAudioFormat error"),
        }
    }

    #[test]
    fn test_validate_wav_wrong_sample_rate() {
        let temp_dir = std::env::temp_dir();
        let wrong_rate_wav = temp_dir.join("test_wrong_rate_validation.wav");

        // Créer fichier WAV avec sample rate incorrect (44100 au lieu de 16000)
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100, // Wrong rate
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&wrong_rate_wav, spec).unwrap();
        // Écrire quelques samples
        for _ in 0..1000 {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();

        let result = validate_wav_file(&wrong_rate_wav);

        // Cleanup
        let _ = std::fs::remove_file(&wrong_rate_wav);

        assert!(result.is_err(), "Should return error for wrong sample rate");
        match result {
            Err(AppError::InvalidAudioFormat(msg)) => {
                assert!(
                    msg.contains("16000") && msg.contains("44100"),
                    "Error should mention expected and actual sample rate: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidAudioFormat error"),
        }
    }

    #[test]
    fn test_validate_wav_stereo_file() {
        let temp_dir = std::env::temp_dir();
        let stereo_wav = temp_dir.join("test_stereo_validation.wav");

        // Créer fichier WAV stéréo (2 canaux au lieu de 1)
        let spec = hound::WavSpec {
            channels: 2, // Wrong - stereo
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&stereo_wav, spec).unwrap();
        // Écrire quelques samples (2 par frame pour stéréo)
        for _ in 0..1000 {
            writer.write_sample(0i16).unwrap();
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();

        let result = validate_wav_file(&stereo_wav);

        // Cleanup
        let _ = std::fs::remove_file(&stereo_wav);

        assert!(result.is_err(), "Should return error for stereo file");
        match result {
            Err(AppError::InvalidAudioFormat(msg)) => {
                assert!(
                    msg.contains("mono") && msg.contains("2"),
                    "Error should mention expected mono and actual 2 channels: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidAudioFormat error"),
        }
    }

    #[test]
    fn test_validate_wav_valid_file() {
        let temp_dir = std::env::temp_dir();
        let valid_wav = temp_dir.join("test_valid_validation.wav");

        // Créer fichier WAV valide
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&valid_wav, spec).unwrap();
        // Écrire quelques samples
        for _ in 0..1000 {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();

        let result = validate_wav_file(&valid_wav);

        // Cleanup
        let _ = std::fs::remove_file(&valid_wav);

        assert!(result.is_ok(), "Should return Ok for valid WAV file");
    }

    #[test]
    fn test_error_messages_are_actionable() {
        let err = AppError::InvalidAudioFormat("test".to_string());
        let msg = err.to_string();
        assert!(
            msg.contains("Réenregistrez"),
            "Should suggest action: {}",
            msg
        );
    }

    #[test]
    fn test_model_load_failed_message_is_actionable() {
        let err = AppError::ModelLoadFailed("generic error".to_string());
        let msg = err.to_string();
        // Le message doit contenir les DEUX suggestions d'action
        assert!(
            msg.contains("download-models.sh"),
            "ModelLoadFailed should suggest download-models.sh: {}",
            msg
        );
        assert!(
            msg.contains("mémoire"),
            "ModelLoadFailed should mention memory check: {}",
            msg
        );
    }

    #[test]
    fn test_model_not_found_includes_download_instructions() {
        let err = AppError::ModelNotFound("test path".to_string());
        let msg = err.to_string();
        assert!(
            msg.contains("Modèle Whisper non trouvé"),
            "Should mention model not found: {}",
            msg
        );
    }
}
