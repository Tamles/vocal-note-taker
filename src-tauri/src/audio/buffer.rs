//! Audio buffer module - WAV file writing
//!
//! Gère l'écriture des samples audio vers un fichier WAV format whisper.cpp.

use hound::{WavSpec, WavWriter};
use std::fs;
use std::path::PathBuf;

use crate::error::AppError;

/// Spécification WAV pour whisper.cpp (16kHz mono 16-bit)
fn get_wav_spec(sample_rate: u32) -> WavSpec {
    WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    }
}

/// Retourne le chemin du dossier temporaire de l'application
pub fn get_temp_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("vocal-note-taker")
        .join("temp")
}

/// Retourne le chemin du fichier WAV temporaire
pub fn get_wav_path() -> PathBuf {
    get_temp_dir().join("recording.wav")
}

/// Sauvegarde les samples audio dans un fichier WAV
///
/// # Arguments
/// * `samples` - Samples audio en f32 (-1.0 à 1.0)
/// * `sample_rate` - Taux d'échantillonnage (ex: 16000)
///
/// # Returns
/// Chemin du fichier WAV créé
///
/// # Errors
/// - `IoError` si création dossier ou fichier échoue
pub fn save_wav(samples: &[f32], sample_rate: u32) -> Result<PathBuf, AppError> {
    let temp_dir = get_temp_dir();

    // Créer le dossier temp si inexistant
    fs::create_dir_all(&temp_dir)?;

    let wav_path = get_wav_path();
    let spec = get_wav_spec(sample_rate);

    let mut writer = WavWriter::create(&wav_path, spec)
        .map_err(|e| AppError::IoError(format!("Cannot create WAV file: {}", e)))?;

    // Convertir f32 (-1.0 à 1.0) vers i16 (-32768 à 32767)
    for &sample in samples {
        let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        writer
            .write_sample(sample_i16)
            .map_err(|e| AppError::IoError(format!("Cannot write WAV sample: {}", e)))?;
    }

    writer
        .finalize()
        .map_err(|e| AppError::IoError(format!("Cannot finalize WAV file: {}", e)))?;

    Ok(wav_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Sous-dossier dédié aux tests buffer (isolé du cleanup_temp_files)
    fn get_test_dir() -> PathBuf {
        get_temp_dir().join("buffer_tests")
    }

    /// Helper pour créer un chemin unique pour les tests
    fn unique_test_path(prefix: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        // Utiliser un ID simple basé sur le hash du thread
        let thread_id = format!("{:?}", std::thread::current().id())
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        // Utiliser sous-dossier dédié pour éviter conflit avec cleanup_temp_files
        get_test_dir().join(format!("{}_{}_{}.wav", prefix, thread_id, timestamp))
    }

    /// Helper pour sauvegarder dans un fichier test unique
    fn save_wav_test(samples: &[f32], sample_rate: u32, path: &PathBuf) -> Result<(), AppError> {
        let test_dir = get_test_dir();
        fs::create_dir_all(&test_dir)?;

        let spec = get_wav_spec(sample_rate);

        let mut writer = WavWriter::create(path, spec)
            .map_err(|e| AppError::IoError(format!("Cannot create WAV file: {}", e)))?;

        for &sample in samples {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer
                .write_sample(sample_i16)
                .map_err(|e| AppError::IoError(format!("Cannot write WAV sample: {}", e)))?;
        }

        writer
            .finalize()
            .map_err(|e| AppError::IoError(format!("Cannot finalize WAV file: {}", e)))?;

        Ok(())
    }

    #[test]
    fn test_wav_spec() {
        let spec = get_wav_spec(16000);
        assert_eq!(spec.channels, 1);
        assert_eq!(spec.sample_rate, 16000);
        assert_eq!(spec.bits_per_sample, 16);
    }

    #[test]
    fn test_get_temp_dir_returns_valid_path() {
        let temp_dir = get_temp_dir();
        assert!(temp_dir.to_string_lossy().contains("vocal-note-taker"));
        assert!(temp_dir.to_string_lossy().contains("temp"));
    }

    #[test]
    fn test_get_wav_path_returns_valid_path() {
        let wav_path = get_wav_path();
        assert!(wav_path.to_string_lossy().ends_with("recording.wav"));
    }

    #[test]
    fn test_save_wav_creates_file() {
        let path = unique_test_path("test_creates");
        let samples: Vec<f32> = vec![0.0; 16000];

        let result = save_wav_test(&samples, 16000, &path);
        assert!(result.is_ok(), "save_wav_test failed: {:?}", result);

        // Verify file exists by attempting to open it (more reliable than exists())
        let file_check = fs::File::open(&path);
        assert!(
            file_check.is_ok(),
            "WAV file should be readable at {:?}: {:?}",
            path,
            file_check.err()
        );

        // Cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_save_wav_correct_format() {
        let path = unique_test_path("test_format");
        let samples: Vec<f32> = vec![0.5, -0.5, 0.0];

        save_wav_test(&samples, 16000, &path).unwrap();

        let reader = hound::WavReader::open(&path).unwrap();
        let spec = reader.spec();

        assert_eq!(spec.channels, 1);
        assert_eq!(spec.sample_rate, 16000);
        assert_eq!(spec.bits_per_sample, 16);

        // Cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_save_wav_sample_conversion() {
        let path = unique_test_path("test_conversion");
        let samples: Vec<f32> = vec![1.0, -1.0, 0.0, 0.5, -0.5];

        save_wav_test(&samples, 16000, &path).unwrap();

        let mut reader = hound::WavReader::open(&path).unwrap();
        let read_samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

        assert_eq!(read_samples.len(), 5);
        assert_eq!(read_samples[0], 32767);  // 1.0 -> 32767
        assert_eq!(read_samples[1], -32767); // -1.0 -> -32767 (clamped)
        assert_eq!(read_samples[2], 0);      // 0.0 -> 0
        assert!((read_samples[3] - 16383).abs() <= 1);  // 0.5 -> ~16383
        assert!((read_samples[4] + 16383).abs() <= 1);  // -0.5 -> ~-16383

        // Cleanup
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_save_wav_production_function() {
        // Test la vraie fonction save_wav (utilise le chemin par défaut)
        let samples: Vec<f32> = vec![0.0; 100];
        let result = save_wav(&samples, 16000);
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.exists());
        assert!(path.to_string_lossy().ends_with("recording.wav"));

        // Cleanup
        let _ = fs::remove_file(path);
    }
}
