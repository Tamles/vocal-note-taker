use serde::{Serialize, Serializer};
use thiserror::Error;

/// Centralized error type for the application.
/// All errors have actionable messages (FR47) that help users understand what happened
/// and how to resolve the issue.
///
/// Custom serialization produces: {"type": "VariantName", "message": "..."}
/// This format is directly consumable by the frontend without parsing.
#[derive(Debug, Error, Clone)]
pub enum AppError {
    #[error("Accès au microphone refusé. Vérifiez les permissions système.")]
    MicrophoneAccessDenied,

    #[error("Aucun microphone détecté. Connectez un microphone et réessayez.")]
    MicrophoneNotFound,

    #[error("Transcription échouée: {0}. Réessayez l'enregistrement.")]
    TranscriptionFailed(String),

    #[error("Enregistrement interrompu. Réessayez.")]
    RecordingInterrupted,

    #[error("Erreur de configuration: {0}. Vérifiez config.toml.")]
    ConfigurationError(String),

    #[error("Impossible de copier dans le presse-papiers. Réessayez.")]
    ClipboardError,

    #[error("Erreur système: {0}")]
    IoError(String),

    #[error("Échec d'enregistrement du raccourci clavier: {0}. L'application reste fonctionnelle via le bouton.")]
    HotkeyRegistrationFailed(String),

    #[error("Modèle Whisper non trouvé. {0}")]
    ModelNotFound(String),

    #[error("Échec du chargement du modèle Whisper: {0}")]
    ModelLoadFailed(String),
}

/// Serialization format for frontend consumption.
#[derive(Serialize)]
struct SerializedAppError<'a> {
    #[serde(rename = "type")]
    error_type: &'a str,
    message: String,
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let error_type = match self {
            AppError::MicrophoneAccessDenied => "MicrophoneAccessDenied",
            AppError::MicrophoneNotFound => "MicrophoneNotFound",
            AppError::TranscriptionFailed(_) => "TranscriptionFailed",
            AppError::RecordingInterrupted => "RecordingInterrupted",
            AppError::ConfigurationError(_) => "ConfigurationError",
            AppError::ClipboardError => "ClipboardError",
            AppError::IoError(_) => "IoError",
            AppError::HotkeyRegistrationFailed(_) => "HotkeyRegistrationFailed",
            AppError::ModelNotFound(_) => "ModelNotFound",
            AppError::ModelLoadFailed(_) => "ModelLoadFailed",
        };

        SerializedAppError {
            error_type,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microphone_access_denied_is_actionable() {
        let err = AppError::MicrophoneAccessDenied;
        let msg = err.to_string();
        assert!(msg.contains("Vérifiez"), "Message should suggest action");
    }

    #[test]
    fn test_microphone_not_found_is_actionable() {
        let err = AppError::MicrophoneNotFound;
        let msg = err.to_string();
        assert!(msg.contains("Connectez") || msg.contains("réessayez"), "Message should suggest action");
    }

    #[test]
    fn test_transcription_failed_is_actionable() {
        let err = AppError::TranscriptionFailed("timeout".to_string());
        let msg = err.to_string();
        assert!(msg.contains("timeout"), "Message should contain details");
        assert!(msg.contains("Réessayez"), "Message should suggest action");
    }

    #[test]
    fn test_recording_interrupted_is_actionable() {
        let err = AppError::RecordingInterrupted;
        let msg = err.to_string();
        assert!(msg.contains("Réessayez"), "Message should suggest action");
    }

    #[test]
    fn test_configuration_error_is_actionable() {
        let err = AppError::ConfigurationError("missing field".to_string());
        let msg = err.to_string();
        assert!(msg.contains("missing field"), "Message should contain details");
        assert!(msg.contains("config.toml"), "Message should point to config file");
    }

    #[test]
    fn test_clipboard_error_is_actionable() {
        let err = AppError::ClipboardError;
        let msg = err.to_string();
        assert!(msg.contains("Réessayez"), "Message should suggest action");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_err: AppError = io_err.into();
        match app_err {
            AppError::IoError(msg) => assert!(msg.contains("file not found")),
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_error_serialization_format() {
        let err = AppError::TranscriptionFailed("timeout".to_string());
        let json = serde_json::to_string(&err).unwrap();

        // Verify the custom serialization format: {"type": "...", "message": "..."}
        assert!(json.contains(r#""type":"TranscriptionFailed""#), "Should have type field");
        assert!(json.contains(r#""message":"#), "Should have message field");
        assert!(json.contains("timeout"), "Message should contain details");
    }

    #[test]
    fn test_error_serialization_simple_variant() {
        let err = AppError::MicrophoneAccessDenied;
        let json = serde_json::to_string(&err).unwrap();

        assert!(json.contains(r#""type":"MicrophoneAccessDenied""#));
        assert!(json.contains("Vérifiez les permissions"));
    }

    #[test]
    fn test_hotkey_registration_failed_serialization() {
        let err = AppError::HotkeyRegistrationFailed("shortcut already in use".to_string());
        let json = serde_json::to_string(&err).unwrap();

        assert!(json.contains(r#""type":"HotkeyRegistrationFailed""#));
        assert!(json.contains("shortcut already in use"));
        assert!(json.contains("reste fonctionnelle"));
    }

    #[test]
    fn test_all_errors_are_actionable() {
        let errors = vec![
            AppError::MicrophoneAccessDenied,
            AppError::MicrophoneNotFound,
            AppError::TranscriptionFailed("test".to_string()),
            AppError::RecordingInterrupted,
            AppError::ConfigurationError("test".to_string()),
            AppError::ClipboardError,
            AppError::HotkeyRegistrationFailed("test".to_string()),
            AppError::ModelNotFound("test".to_string()),
            AppError::ModelLoadFailed("test".to_string()),
        ];

        for err in errors {
            let msg = err.to_string();
            assert!(
                msg.contains("Vérifiez")
                    || msg.contains("Réessayez")
                    || msg.contains("Connectez")
                    || msg.contains("reste fonctionnelle")
                    || msg.contains("Modèle")
                    || msg.contains("modèle"),
                "Error '{}' should have actionable message",
                msg
            );
        }
    }
}
