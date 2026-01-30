//! Clipboard module - clipboard operations
//!
//! Provides clipboard integration via tauri-plugin-clipboard-manager.
//! - Copy text to clipboard (FR20)
//! - Plain text format only (FR24)

use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::error::AppError;

/// Copie le texte dans le presse-papiers système.
///
/// # Arguments
/// * `app` - Handle Tauri pour accéder au plugin clipboard
/// * `text` - Texte à copier (plain text, FR24)
///
/// # Errors
/// Retourne `AppError::ClipboardError` si la copie échoue.
pub fn copy_to_clipboard(app: &AppHandle, text: &str) -> Result<(), AppError> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| {
            eprintln!("Clipboard write failed: {:?}", e);
            AppError::ClipboardError
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Les tests d'intégration du clipboard nécessitent un environnement Tauri complet
    // avec AppHandle valide. Ce n'est pas possible en test unitaire car:
    // 1. AppHandle requiert un runtime Tauri initialisé
    // 2. ClipboardExt::clipboard() nécessite le plugin clipboard-manager enregistré
    // 3. L'accès au clipboard système nécessite un contexte graphique (display)
    //
    // Le test réel se fait via:
    // - Tests manuels (Task 6 de la story)
    // - Tests E2E avec Tauri test harness (hors scope MVP)
    //
    // Les tests ci-dessous vérifient la logique de mapping d'erreur et les types.

    #[test]
    fn test_clipboard_error_type() {
        // Vérifier que AppError::ClipboardError retourne le bon message utilisateur
        let error = AppError::ClipboardError;
        assert_eq!(
            error.to_string(),
            "Impossible de copier dans le presse-papiers. Réessayez."
        );
    }

    #[test]
    fn test_clipboard_error_is_user_friendly() {
        // Le message d'erreur doit être en français et actionnable
        let error = AppError::ClipboardError;
        let msg = error.to_string();
        assert!(msg.contains("presse-papiers"), "Message doit mentionner le presse-papiers");
        assert!(msg.contains("Réessayez"), "Message doit suggérer une action");
    }
}
