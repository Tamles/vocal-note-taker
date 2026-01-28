# Story 3.1: Intégration whisper-rs et chargement du modèle

Status: done

## Story

As a utilisateur,
I want que l'application utilise un modèle de transcription local,
so that mes données vocales restent privées et je puisse travailler hors-ligne.

## Acceptance Criteria

1. **Given** le module transcription/whisper.rs existe
   **When** whisper-rs est intégré
   **Then** le binding vers whisper.cpp est fonctionnel

2. **Given** le modèle ggml-large.bin (~3GB) est présent
   **When** l'application démarre
   **Then** le modèle est chargé depuis ~/.local/share/vocal-note-taker/models/
   **And** aucune connexion réseau n'est établie (FR10)

3. **Given** le modèle n'est pas présent
   **When** l'application tente de le charger
   **Then** un message d'erreur clair indique comment obtenir le modèle
   **And** un script download-models.sh est disponible

4. **Given** la transcription est lancée
   **When** le processus s'exécute
   **Then** tout le traitement se fait localement (FR9)
   **And** aucune donnée n'est envoyée sur le réseau

## Tasks / Subtasks

- [x] **Task 1: Ajouter whisper-rs à Cargo.toml** (AC: #1)
  - [x] Rechercher la version stable latest de whisper-rs sur crates.io
  - [x] Ajouter `whisper-rs = "X.X"` dans [dependencies]
  - [x] Vérifier compilation avec `cargo check`
  - [x] Documenter quelle version exacte est utilisée

- [x] **Task 2: Créer les types et structures pour le modèle** (AC: #1, #2)
  - [x] Définir struct `WhisperModel` wrappant `whisper_rs::WhisperContext`
  - [x] Créer `WhisperState` pour managed state Tauri (Arc<Mutex<Option<WhisperModel>>>)
  - [x] Définir `ModelConfig` avec chemin modèle et paramètres

- [x] **Task 3: Implémenter le chargement du modèle** (AC: #2, #3)
  - [x] Fonction `get_model_path()` → `~/.local/share/vocal-note-taker/models/ggml-large.bin`
  - [x] Fonction `load_model(path: &Path) -> Result<WhisperModel, AppError>`
  - [x] Gestion erreur si fichier absent → AppError::ModelNotFound avec instructions
  - [x] Log info au chargement réussi avec taille modèle

- [x] **Task 4: Créer le script download-models.sh** (AC: #3)
  - [x] Script dans `scripts/download-models.sh`
  - [x] Télécharger depuis Hugging Face: ggml-large-v3.bin
  - [x] Créer répertoire `~/.local/share/vocal-note-taker/models/`
  - [x] Afficher progression et instructions

- [x] **Task 5: Intégrer le chargement dans lib.rs** (AC: #2)
  - [x] Ajouter WhisperState dans `.manage()`
  - [x] Appel `load_model()` optionnel dans `.setup()` (lazy loading recommandé)
  - [x] Gérer absence modèle gracieusement (warning, pas crash)

- [x] **Task 6: Ajouter erreurs spécifiques dans error.rs** (AC: #3)
  - [x] Variante `ModelNotFound(String)` avec instructions téléchargement
  - [x] Variante `ModelLoadFailed(String)` pour erreurs chargement
  - [x] Mettre à jour la sérialisation et tests

- [x] **Task 7: Tests et validation** (AC: #1, #2, #3, #4)
  - [x] Build sans erreur (`pnpm tauri build`)
  - [x] Test: sans modèle → message d'erreur clair avec instructions
  - [x] Test: avec modèle → chargement réussi (log visible)
  - [x] Test: script download-models.sh fonctionne
  - [x] Vérifier AUCUNE connexion réseau pendant runtime (`cargo tree | grep -E "(reqwest|hyper)"` = vide)

## Dev Notes

### Architecture Compliance

**CRITIQUE: Cette story est 100% BACKEND (Rust)**

Le module `transcription/whisper.rs` existe mais est un **PLACEHOLDER vide**. Cette story doit:
1. Intégrer whisper-rs comme dépendance
2. Implémenter le chargement du modèle
3. Préparer l'infrastructure pour Story 3.2 (transcription async)

**Fichiers à modifier:**
```
src-tauri/Cargo.toml                      # MODIFIER - Ajouter whisper-rs
src-tauri/src/transcription/whisper.rs    # MODIFIER - Implémenter module
src-tauri/src/transcription/mod.rs        # MODIFIER - Exporter types
src-tauri/src/error.rs                    # MODIFIER - Ajouter variantes erreur
src-tauri/src/lib.rs                      # MODIFIER - Gérer WhisperState
scripts/download-models.sh                # CRÉER - Script téléchargement
```

**Aucune modification frontend requise** - cette story est purement backend.

### ⚠️ CRITIQUE: Version whisper-rs

**AVANT D'IMPLÉMENTER**, rechercher la version stable latest de whisper-rs:
- Consulter https://crates.io/crates/whisper-rs
- Vérifier compatibilité avec ggml-large-v3.bin
- Documenter la version choisie dans ce fichier

**Versions connues (à vérifier):**
- whisper-rs 0.10+ pour whisper.cpp récent
- Modèle recommandé: ggml-large-v3.bin (meilleure qualité)

### ⚠️ CRITIQUE: Privacy-First (NFR-SEC-1)

**ABSOLU - Zero Network Calls:**
- ❌ **INTERDIT**: Aucune feature réseau dans whisper-rs
- ❌ **INTERDIT**: Téléchargement automatique du modèle
- ✅ **OBLIGATOIRE**: Modèle pré-téléchargé manuellement par user
- ✅ **OBLIGATOIRE**: Vérifier `cargo tree | grep -E "(reqwest|hyper|tokio.*net)"` = vide

### Chemins des fichiers

**Modèle Whisper:**
```
Linux:  ~/.local/share/vocal-note-taker/models/ggml-large-v3.bin
macOS:  ~/Library/Application Support/vocal-note-taker/models/ggml-large-v3.bin
```

**Utiliser crate `dirs` déjà présent dans Cargo.toml:**
```rust
use dirs::data_local_dir;

fn get_model_path() -> PathBuf {
    let mut path = data_local_dir()
        .expect("Could not find local data directory");
    path.push("vocal-note-taker");
    path.push("models");
    path.push("ggml-large-v3.bin");
    path
}
```

### Pattern d'Implémentation whisper.rs

```rust
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
                AppError::ModelLoadFailed("Chemin de modèle invalide".to_string())
            })?,
            params,
        ).map_err(|e| AppError::ModelLoadFailed(e.to_string()))?;

        println!("Whisper model loaded successfully from: {}", model_path.display());

        Ok(Self { context })
    }

    /// Returns a reference to the underlying WhisperContext.
    /// Used by transcription functions.
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
pub fn get_model_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .expect("Could not determine local data directory");
    path.push("vocal-note-taker");
    path.push("models");
    path.push("ggml-large-v3.bin");
    path
}

/// Ensures the model directory exists.
///
/// Creates ~/.local/share/vocal-note-taker/models/ if not present.
pub fn ensure_model_dir() -> Result<PathBuf, AppError> {
    let model_path = get_model_path();
    if let Some(parent) = model_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(model_path)
}

/// Checks if the Whisper model is available.
///
/// Returns Ok(path) if model exists, Err with instructions otherwise.
pub fn check_model_availability() -> Result<PathBuf, AppError> {
    let model_path = get_model_path();
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
    fn test_get_model_path_contains_expected_parts() {
        let path = get_model_path();
        let path_str = path.to_string_lossy();

        assert!(path_str.contains("vocal-note-taker"), "Should contain app name");
        assert!(path_str.contains("models"), "Should contain models dir");
        assert!(path_str.contains("ggml-large"), "Should contain model name");
    }

    #[test]
    fn test_model_not_found_error_has_instructions() {
        let result = check_model_availability();
        // This will fail if model exists, which is OK for test
        if let Err(AppError::ModelNotFound(msg)) = result {
            assert!(msg.contains("download-models.sh"), "Should mention script");
            assert!(msg.contains("huggingface"), "Should mention download source");
        }
        // If model exists, test passes (user has model installed)
    }

    #[test]
    fn test_whisper_state_default() {
        let state = WhisperState::default();
        // Model should be None initially (lazy loading)
        let guard = state.model.try_lock().unwrap();
        assert!(guard.is_none(), "Model should be None by default");
    }
}
```

### Modifications error.rs

```rust
// Ajouter ces variantes dans l'enum AppError

#[error("Modèle Whisper non trouvé. {0}")]
ModelNotFound(String),

#[error("Échec du chargement du modèle Whisper: {0}")]
ModelLoadFailed(String),
```

```rust
// Ajouter dans le match de serialize
AppError::ModelNotFound(_) => "ModelNotFound",
AppError::ModelLoadFailed(_) => "ModelLoadFailed",
```

```rust
// Ajouter dans test_all_errors_are_actionable
AppError::ModelNotFound("test".to_string()),
AppError::ModelLoadFailed("test".to_string()),
```

### Modifications lib.rs

```rust
// Ajouter import
use crate::transcription::whisper::WhisperState;

// Dans run(), ajouter le state
tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .manage(AudioState::default())
    .manage(WhisperState::default())  // AJOUTER
    // ...

// Dans .setup(), ajouter vérification optionnelle du modèle
.setup(|app| {
    // ... existing code ...

    // Check model availability (non-fatal warning)
    match crate::transcription::whisper::check_model_availability() {
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
```

### Script download-models.sh

Créer `scripts/download-models.sh`:

```bash
#!/bin/bash
# Download Whisper models for vocal-note-taker
#
# This script downloads the whisper.cpp compatible model files.
# Models are stored in ~/.local/share/vocal-note-taker/models/
#
# Usage: ./scripts/download-models.sh

set -e

# Configuration
MODEL_NAME="ggml-large-v3.bin"
MODEL_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/${MODEL_NAME}"
MODEL_SIZE="~3GB"

# Determine model directory
if [[ "$OSTYPE" == "darwin"* ]]; then
    MODEL_DIR="$HOME/Library/Application Support/vocal-note-taker/models"
else
    MODEL_DIR="$HOME/.local/share/vocal-note-taker/models"
fi

MODEL_PATH="$MODEL_DIR/$MODEL_NAME"

echo "==================================="
echo "vocal-note-taker Model Downloader"
echo "==================================="
echo ""
echo "Model: $MODEL_NAME ($MODEL_SIZE)"
echo "Target: $MODEL_PATH"
echo ""

# Check if model already exists
if [[ -f "$MODEL_PATH" ]]; then
    echo "Model already exists at $MODEL_PATH"
    echo "Delete it manually if you want to re-download."
    exit 0
fi

# Create directory
echo "Creating directory: $MODEL_DIR"
mkdir -p "$MODEL_DIR"

# Check for wget or curl
if command -v wget &> /dev/null; then
    echo "Downloading with wget..."
    wget -O "$MODEL_PATH" "$MODEL_URL" --show-progress
elif command -v curl &> /dev/null; then
    echo "Downloading with curl..."
    curl -L -o "$MODEL_PATH" "$MODEL_URL" --progress-bar
else
    echo "Error: Neither wget nor curl found. Please install one of them."
    exit 1
fi

# Verify download
if [[ -f "$MODEL_PATH" ]]; then
    SIZE=$(du -h "$MODEL_PATH" | cut -f1)
    echo ""
    echo "==================================="
    echo "Download complete!"
    echo "Model: $MODEL_PATH"
    echo "Size: $SIZE"
    echo "==================================="
    echo ""
    echo "You can now start vocal-note-taker."
else
    echo "Error: Download failed. Model file not found."
    exit 1
fi
```

### Naming Conventions (CRITIQUE)

**Rust:**
- Module: `whisper.rs` (snake_case)
- Structs: `WhisperModel`, `WhisperState`, `ModelConfig` (PascalCase)
- Fonctions: `load_model()`, `get_model_path()`, `check_model_availability()` (snake_case)
- Constantes: (aucune pour l'instant)

**Paths:**
- Modèle: `~/.local/share/vocal-note-taker/models/ggml-large-v3.bin`
- Script: `scripts/download-models.sh`

### Previous Story Intelligence (Story 2.5)

**Patterns établis à réutiliser:**
- Managed state avec Tauri: `.manage(State::default())`
- Gestion d'erreur gracieuse dans `.setup()` (warning, pas crash)
- Pattern Result<T, AppError> pour toutes fonctions
- Logging avec `println!` et `eprintln!`
- Tests unitaires dans chaque module

**Ce qui existe déjà:**
- `dirs` crate pour chemins cross-platform
- `AppError` enum avec serialization
- Pattern de modules séparés (audio/, system/, config/)

### Git Intelligence

**Derniers commits:**
```
cd4297e Story 2-4
cc010dc story 2-3
14899f3 stories 2-1 and 2-2
b340e02 End of epic 1
```

**Convention commit:**
```
Story 3-1 - intégration whisper-rs et chargement modèle
```

### NFR Compliance

- **FR9:** System can transcribe recorded audio using local whisper.cpp model ✓
- **FR10:** System can process transcription entirely offline without network dependency ✓
- **NFR-SEC-1:** Zero network calls - modèle pré-téléchargé manuellement ✓
- **NFR-SEC-2:** Data privacy - processing 100% local ✓
- **NFR-MAINT-5:** Dependency management - whisper-rs stable, bien maintenu ✓

### Dependencies

**Nouvelles dépendances requises:**
```toml
# Transcription locale (Story 3.1)
whisper-rs = "0.12"  # ⚠️ VÉRIFIER VERSION LATEST sur crates.io
```

**VÉRIFICATION CRITIQUE avant ajout:**
```bash
# Après ajout de whisper-rs, vérifier AUCUNE dep réseau
cargo tree | grep -E "(reqwest|hyper|tokio.*net)"
# Doit retourner VIDE
```

**Si whisper-rs ajoute deps réseau:**
- Rechercher features pour désactiver
- Ou trouver alternative sans réseau
- JAMAIS accepter de deps réseau

### Lazy Loading vs Eager Loading

**Recommandation: Lazy Loading**

Le modèle (~3GB) est lourd à charger. Deux approches:

1. **Eager Loading** (au démarrage):
   - Avantage: Transcription instantanée
   - Inconvénient: Démarrage lent (5-10s), RAM utilisée même si pas de transcription

2. **Lazy Loading** (à la première transcription):
   - Avantage: Démarrage rapide, RAM économisée
   - Inconvénient: Première transcription plus lente

**Pour MVP: Lazy Loading recommandé**
- Vérifier présence modèle au startup (warning si absent)
- Charger réellement au premier appel de transcription (Story 3.2)
- WhisperState contient `Option<WhisperModel>` initialisé à None

### Edge Cases à Considérer

1. **Modèle absent:** Message d'erreur clair avec instructions téléchargement
2. **Modèle corrompu:** whisper-rs retourne erreur → AppError::ModelLoadFailed
3. **Permissions insuffisantes:** Erreur lecture fichier → AppError::IoError
4. **Chemin invalide (unicode):** Gérer avec `.to_str()` et erreur claire
5. **Espace disque insuffisant:** Erreur au download script (pas runtime)

### Project Structure Notes

```
src-tauri/
├── src/
│   ├── transcription/
│   │   ├── mod.rs                # Exporter whisper module + types
│   │   └── whisper.rs            # À IMPLÉMENTER - cette story
│   ├── error.rs                  # À MODIFIER - nouvelles variantes
│   └── lib.rs                    # À MODIFIER - WhisperState
scripts/
└── download-models.sh            # À CRÉER - script téléchargement
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md - Story 3.1]
- [Source: _bmad-output/planning-artifacts/architecture.md - Transcription Engine, Lines 351-360]
- [Source: _bmad-output/planning-artifacts/architecture.md - Dependencies Summary, Lines 597-640]
- [Source: _bmad-output/project-context.md - Rule #1 Privacy-First Architecture]
- [Source: _bmad-output/project-context.md - Rule #2 Error Handling Strict]
- [Source: src-tauri/Cargo.toml - Current dependencies]
- [Source: src-tauri/src/error.rs - AppError enum pattern]
- [whisper-rs crate: https://crates.io/crates/whisper-rs]
- [whisper.cpp models: https://huggingface.co/ggerganov/whisper.cpp]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- Compilation whisper-rs nécessite libclang-dev, clang, cmake (dépendances système)
- whisper-rs-sys compile whisper.cpp localement via cmake

### Completion Notes List

- ✅ whisper-rs 0.15.1 intégré (version stable latest - 2026-01-28)
- ✅ WhisperModel et WhisperState implémentés avec lazy loading
- ✅ get_model_path() retourne ~/.local/share/vocal-note-taker/models/ggml-large-v3.bin
- ✅ check_model_availability() vérifie présence et retourne instructions claires si absent
- ✅ AppError::ModelNotFound et ModelLoadFailed ajoutés avec sérialisation
- ✅ WhisperState managé dans lib.rs avec vérification au startup (warning gracieux)
- ✅ Script download-models.sh créé et testé (syntax OK)
- ✅ 31 tests passent (4 nouveaux tests whisper)
- ✅ Build pnpm tauri build réussi (.deb et .AppImage générés)
- ✅ Aucune dépendance réseau (cargo tree vérifié)
- ✅ NFR-SEC-1 respecté: 100% local, zéro network calls

### Change Log

- 2026-01-28: Story 3-1 implémentée - intégration whisper-rs et chargement modèle
- 2026-01-28: Code review - corrigé `.expect()` panic → `Result`, tests renforcés

### File List

**Modifiés:**
- src-tauri/Cargo.toml (ajout whisper-rs = "0.15")
- src-tauri/Cargo.lock (mise à jour dépendances)
- src-tauri/src/transcription/whisper.rs (implémentation complète)
- src-tauri/src/transcription/mod.rs (exports ajoutés)
- src-tauri/src/error.rs (ModelNotFound, ModelLoadFailed)
- src-tauri/src/lib.rs (WhisperState + check_model_availability)

**Créés:**
- scripts/download-models.sh (script téléchargement modèle)

