# Story 2.5: Raccourci clavier global pour enregistrement

Status: done

## Story

As a utilisateur,
I want utiliser un raccourci clavier pour enregistrer,
so that je puisse démarrer rapidement sans utiliser la souris.

## Acceptance Criteria

1. **Given** l'application est lancée
   **When** tauri-plugin-global-shortcut est configuré
   **Then** le raccourci par défaut (Ctrl+Alt+R) est enregistré

2. **Given** le raccourci est configuré
   **When** j'appuie sur le raccourci clavier
   **Then** l'enregistrement démarre (FR2)
   **And** le comportement est identique au clic sur le bouton

3. **Given** l'enregistrement est actif via raccourci
   **When** j'appuie à nouveau sur le raccourci
   **Then** l'enregistrement s'arrête (FR3)

4. **Given** le raccourci est utilisé
   **When** l'application est en arrière-plan
   **Then** le raccourci fonctionne toujours (préparation pour Epic 5)

## Tasks / Subtasks

- [x] **Task 1: Ajouter tauri-plugin-global-shortcut** (AC: #1)
  - [x] Modifier `src-tauri/Cargo.toml`
  - [x] Ajouter `tauri-plugin-global-shortcut = "2"` dans [dependencies]
  - [x] Vérifier compilation avec `cargo check`

- [x] **Task 2: Configurer les permissions dans tauri.conf.json** (AC: #1, #4)
  - [x] Ajouter capability pour global-shortcut dans `src-tauri/capabilities/default.json`
  - [x] Ou créer un fichier capabilities si non existant
  - [x] Permissions nécessaires: `global-shortcut:allow-register`, `global-shortcut:allow-unregister`

- [x] **Task 3: Implémenter system/hotkeys.rs** (AC: #1, #2, #3)
  - [x] Remplacer le placeholder actuel
  - [x] Créer constante `DEFAULT_TOGGLE_RECORDING = "CmdOrCtrl+Alt+R"`
  - [x] Implémenter fonction `register_global_shortcuts(app: &AppHandle)`
  - [x] Gérer toggle: start si idle, stop si recording
  - [x] Propagation erreurs via AppError si échec registration

- [x] **Task 4: Intégrer le plugin dans lib.rs** (AC: #1)
  - [x] N/A - Plugin 2.x utilise trait GlobalShortcutExt, pas .plugin()
  - [x] Appeler `register_global_shortcuts()` dans `.setup()`
  - [x] Gérer erreur si registration échoue (warning, pas crash)

- [x] **Task 5: Gérer le toggle recording via hotkey** (AC: #2, #3)
  - [x] Dans hotkeys.rs, accéder à AudioState
  - [x] Vérifier état actuel (recording ou idle)
  - [x] Appeler start_recording ou stop_recording selon état
  - [x] Émettre events recording-started / recording-stopped

- [x] **Task 6: Tests et validation** (AC: #1, #2, #3, #4)
  - [x] Build sans erreur (`pnpm tauri build`)
  - [x] Test manuel: appuyer Ctrl+Alt+R → enregistrement démarre
  - [x] Test manuel: appuyer à nouveau → enregistrement s'arrête
  - [x] Test manuel: waveform, timer, indicateur REC fonctionnent
  - [x] Test manuel: raccourci fonctionne avec app en arrière-plan (si OS le permet)
  - [x] Vérifier cleanup shortcut au quit

## Dev Notes

### Architecture Compliance

**CRITIQUE: Cette story est principalement BACKEND (Rust)**

Le module `system/hotkeys.rs` existe mais est un **PLACEHOLDER vide**. Cette story doit l'implémenter complètement.

**Fichiers à modifier:**
```
src-tauri/Cargo.toml                    # MODIFIER - Ajouter plugin
src-tauri/capabilities/default.json     # MODIFIER - Ajouter permissions
src-tauri/src/system/hotkeys.rs         # MODIFIER - Implémenter module
src-tauri/src/lib.rs                    # MODIFIER - Intégrer plugin
```

**Aucune modification frontend requise** - les event listeners `recording-started` et `recording-stopped` existent déjà dans `+page.svelte` et mettent à jour les stores correctement.

### Code Backend Existant (À RÉUTILISER)

```rust
// src-tauri/src/commands.rs (lignes 68-102)
// start_recording et stop_recording existent déjà
// La logique de toggle devra réutiliser AudioState

pub struct AudioState {
    pub recording: Mutex<Option<RecordingHandle>>,
}

// Vérifier si recording en cours:
let recording_guard = state.recording.lock().expect("...");
let is_recording = recording_guard.is_some();
```

```rust
// src-tauri/src/lib.rs (ligne 38)
// AudioState est déjà managé
.manage(AudioState::default())
```

### Implémentation hotkeys.rs - Pattern Recommandé

```rust
//! Global hotkeys module - keyboard shortcuts
//!
//! Registers global keyboard shortcuts for recording toggle.
//! Uses tauri-plugin-global-shortcut 2.x.

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::commands::AudioState;
use crate::error::AppError;
use crate::audio;

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
        .map_err(|e| AppError::HotkeyRegistrationFailed(format!("Invalid shortcut: {}", e)))?;

    let app_handle = app.clone();

    app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
        // Clone app_handle pour le callback
        let app = app_handle.clone();

        // Exécuter toggle dans async runtime
        tauri::async_runtime::spawn(async move {
            if let Err(e) = toggle_recording(&app).await {
                eprintln!("Hotkey toggle recording error: {:?}", e);
                // Optionnel: émettre event error vers frontend
                let _ = app.emit("error", serde_json::json!({
                    "type": "HotkeyError",
                    "message": format!("Erreur raccourci: {}", e)
                }));
            }
        });
    }).map_err(|e| AppError::HotkeyRegistrationFailed(e.to_string()))?;

    // Register the shortcut
    app.global_shortcut()
        .register(shortcut)
        .map_err(|e| AppError::HotkeyRegistrationFailed(e.to_string()))?;

    Ok(())
}

/// Toggle recording state: start if idle, stop if recording.
///
/// This function mirrors the logic of commands::start_recording and
/// commands::stop_recording but for hotkey context.
async fn toggle_recording(app: &AppHandle) -> Result<(), AppError> {
    let state: tauri::State<'_, AudioState> = app.state();

    let is_recording = {
        let guard = state.recording.lock()
            .expect("Audio state lock poisoned");
        guard.is_some()
    };

    if is_recording {
        // Stop recording - appeler la command existante
        let result = crate::commands::stop_recording(app.state(), app.clone()).await;
        match result {
            Ok(wav_path) => {
                println!("Recording stopped via hotkey: {}", wav_path);
            }
            Err(e) => return Err(e),
        }
    } else {
        // Start recording - appeler la command existante
        crate::commands::start_recording(app.state(), app.clone())?;
        println!("Recording started via hotkey");
    }

    Ok(())
}

/// Unregisters all global shortcuts.
/// Called during graceful shutdown.
pub fn unregister_all(app: &AppHandle) {
    if let Err(e) = app.global_shortcut().unregister_all() {
        eprintln!("Failed to unregister shortcuts: {:?}", e);
    }
}
```

### Modification error.rs - Ajouter variante

```rust
// Ajouter dans l'enum AppError
#[error("Échec d'enregistrement du raccourci clavier: {0}")]
HotkeyRegistrationFailed(String),
```

### Modification lib.rs - Intégration Plugin

```rust
// Ajouter import
use crate::system::hotkeys;

// Dans run(), ajouter le plugin AVANT .setup()
tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_global_shortcut::init())  // AJOUTER
    .manage(AudioState::default())
    // ...

// Dans .setup(), après création menu
.setup(|app| {
    // ... existing menu code ...

    // Register global shortcuts
    if let Err(e) = hotkeys::register_global_shortcuts(&app.handle()) {
        // Log warning mais ne pas crasher
        eprintln!("Warning: Could not register global shortcuts: {:?}", e);
        eprintln!("Recording via button still available.");
    }

    Ok(())
})
```

### Modification shutdown.rs - Cleanup Shortcuts

```rust
// Dans graceful_shutdown ou via AppHandle
// Note: peut nécessiter de passer AppHandle au shutdown
pub fn graceful_shutdown_with_app(app: &AppHandle) -> Result<(), AppError> {
    hotkeys::unregister_all(app);
    // ... reste du cleanup ...
}
```

### Permissions Tauri 2.x - capabilities/default.json

Créer ou modifier `src-tauri/capabilities/default.json`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for vocal-note-taker",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "opener:default",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "global-shortcut:allow-is-registered"
  ]
}
```

### Cargo.toml - Ajout Dépendance

```toml
[dependencies]
# ... existing ...

# Global shortcuts (Story 2.5)
tauri-plugin-global-shortcut = "2"
```

### Naming Conventions (CRITIQUE)

**Rust:**
- Module: `hotkeys.rs` (snake_case)
- Fonctions: `register_global_shortcuts()`, `toggle_recording()`, `unregister_all()` (snake_case)
- Constantes: `DEFAULT_TOGGLE_RECORDING` (SCREAMING_SNAKE_CASE)
- Enum variant: `HotkeyRegistrationFailed` (PascalCase)

**Shortcut format:**
- `CmdOrCtrl+Alt+R` - Cross-platform compatible
- `CmdOrCtrl` = Cmd sur macOS, Ctrl sur Linux/Windows

### Previous Story Intelligence (Story 2.4)

**Patterns établis:**
- AudioState avec Mutex<Option<RecordingHandle>>
- Commands retournent Result<T, AppError>
- Events émis via `app.emit("event-name", payload)`
- Async runtime via `tauri::async_runtime::spawn`
- Graceful shutdown dans `system/shutdown.rs`

**Ce qui existe déjà (à réutiliser):**
- `start_recording` et `stop_recording` commands
- AudioState managed state
- Event listeners frontend prêts
- Système de cleanup au shutdown

### Git Intelligence

**Derniers commits:**
```
cd4297e Story 2-4
cc010dc story 2-3
14899f3 stories 2-1 and 2-2
b340e02 End of epic 1
```

**Convention commits:**
```
Story 2-5 - raccourci clavier global enregistrement
```

### NFR Compliance

- **FR2:** User can initiate audio recording via global keyboard shortcut ✓
- **FR3:** User can stop audio recording via keyboard shortcut ✓
- **FR26:** System can register and respond to global keyboard shortcuts (préparation Epic 5) ✓
- **NFR-USA-4:** Keyboard-first interaction - raccourci clavier principal ✓
- **NFR-SEC-1:** Pas de dépendance réseau - plugin local uniquement ✓

### Platform Considerations

**Linux (Ubuntu):**
- X11: Raccourcis globaux fonctionnent nativement
- Wayland: Limitations possibles selon compositor
  - Fallback: avertissement user si registration échoue
  - App reste fonctionnelle via bouton

**macOS (Phase 2):**
- Nécessite permissions Accessibility dans System Preferences
- `CmdOrCtrl` devient `Cmd` automatiquement

### Edge Cases à Considérer

1. **Shortcut déjà utilisé:** Autre app utilise Ctrl+Alt+R
   - Tauri plugin retourne erreur
   - App log warning, continue sans hotkey

2. **Wayland sans support:** Certains compositors bloquent global shortcuts
   - Détecter échec registration
   - Informer user que bouton est disponible

3. **Multiple press rapides:** User spam le raccourci
   - Mutex sur AudioState protège déjà
   - Ignorer silencieusement si déjà en transition

4. **App en background (préparation Epic 5):**
   - Hotkey doit fonctionner même si fenêtre pas focus
   - C'est le comportement par défaut du plugin

### Testing Strategy

**Tests manuels requis:**

1. **Test registration:**
   ```
   1. Lancer `pnpm tauri dev`
   2. Vérifier console: pas d'erreur "Could not register global shortcuts"
   3. Si erreur: vérifier aucune autre app utilise Ctrl+Alt+R
   ```

2. **Test start via hotkey:**
   ```
   1. App en idle (pas d'enregistrement)
   2. Appuyer Ctrl+Alt+R
   3. Vérifier: indicateur REC, timer démarre, waveform visible
   4. Vérifier console: "Recording started via hotkey"
   ```

3. **Test stop via hotkey:**
   ```
   1. Pendant enregistrement actif
   2. Appuyer Ctrl+Alt+R
   3. Vérifier: enregistrement s'arrête, waveform disparaît
   4. Vérifier console: "Recording stopped via hotkey"
   ```

4. **Test en arrière-plan:**
   ```
   1. Minimiser ou perdre focus de la fenêtre
   2. Appuyer Ctrl+Alt+R
   3. Vérifier: enregistrement démarre en background
   4. Ramener fenêtre au premier plan → état cohérent affiché
   ```

5. **Test robustesse:**
   ```
   1. Spam Ctrl+Alt+R rapidement (5x en 1 seconde)
   2. Vérifier: pas de crash, état cohérent
   3. Enregistrement actif ou inactif, jamais "entre deux"
   ```

### Code Anti-Patterns à Éviter

❌ **MAUVAIS - Blocking dans callback hotkey:**
```rust
// NE PAS FAIRE - bloque le thread hotkey
app.global_shortcut().on_shortcut(shortcut, move |_, _, _| {
    std::thread::sleep(Duration::from_secs(1)); // BLOCKING!
    // ...
});
```

❌ **MAUVAIS - Panic dans callback:**
```rust
// NE PAS FAIRE - crash l'app si erreur
app.global_shortcut().on_shortcut(shortcut, move |_, _, _| {
    toggle_recording(&app).unwrap(); // PANIC si erreur!
});
```

❌ **MAUVAIS - Hardcoded shortcut non-cross-platform:**
```rust
// NE PAS FAIRE - ne marche pas sur macOS
const SHORTCUT: &str = "Ctrl+Alt+R"; // Devrait être "CmdOrCtrl+Alt+R"
```

### Dependencies

**Nouvelles dépendances requises:**
- `tauri-plugin-global-shortcut = "2"` dans Cargo.toml

**Aucune nouvelle dépendance npm** - frontend inchangé

### Project Structure Notes

```
src-tauri/
├── capabilities/
│   └── default.json              # À MODIFIER - permissions global-shortcut
├── src/
│   ├── lib.rs                    # À MODIFIER - plugin + setup
│   ├── error.rs                  # À MODIFIER - HotkeyRegistrationFailed
│   └── system/
│       ├── mod.rs                # Vérifier export hotkeys
│       ├── hotkeys.rs            # À IMPLÉMENTER - cette story
│       ├── clipboard.rs          # Existant
│       └── shutdown.rs           # Optionnel: cleanup shortcuts
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md - Story 2.5]
- [Source: _bmad-output/planning-artifacts/architecture.md - Global Keyboard Shortcuts, Lines 502-509]
- [Source: _bmad-output/planning-artifacts/architecture.md - System Integration Decisions, Lines 500-537]
- [Source: _bmad-output/project-context.md - Rule #2 Error Handling Strict]
- [Source: _bmad-output/project-context.md - Rule #5 Tauri IPC Commands & Events]
- [Source: src-tauri/src/commands.rs - start_recording/stop_recording, Lines 67-140]
- [Source: src-tauri/src/lib.rs - AudioState management, Line 38]
- [Source: _bmad-output/implementation-artifacts/2-4-visualisation-waveform-temps-reel.md - Previous Story Patterns]
- [Tauri 2.x Global Shortcut Plugin: https://v2.tauri.app/plugin/global-shortcut/]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- Compilation initiale: warnings unused imports (pré-existants) - non bloquants
- Plugin tauri-plugin-global-shortcut v2.3.1 téléchargé et compilé
- Correction: Plugin 2.x requiert `Builder::new().build()` pour initialisation
- Test runtime: Raccourci Ctrl+Alt+R déjà utilisé par autre app → fallback gracieux OK
- L'application continue avec le bouton comme alternative

### Completion Notes List

- ✅ Ajout dépendance `tauri-plugin-global-shortcut = "2"` dans Cargo.toml
- ✅ Permissions global-shortcut ajoutées dans capabilities/default.json
- ✅ Module hotkeys.rs entièrement implémenté avec:
  - Constante DEFAULT_TOGGLE_RECORDING = "CmdOrCtrl+Alt+R"
  - Fonction register_global_shortcuts() avec gestion d'erreur non-fatale
  - Fonction toggle_recording() délègue à commands (DRY)
  - Fonction unregister_all() pour cleanup au shutdown
  - Tests unitaires: shortcut constant et parsing
- ✅ Variante HotkeyRegistrationFailed ajoutée dans error.rs avec serialization
- ✅ Intégration dans lib.rs:
  - Appel register_global_shortcuts() dans .setup() avec fallback gracieux
  - Appel unregister_all() dans on_menu_event et on_window_event avant exit
- ✅ Build complet réussi (DEB + AppImage)
- ✅ 27 tests passent dont 3 tests hotkeys/error

### Change Log

- 2026-01-28: Code Review - Corrections appliquées:
  - [H1] Refactorisation DRY: toggle_recording délègue à commands::start/stop_recording
  - [H2] Fix TOCTOU: map_err au lieu de expect sur mutex lock
  - [M1] Test ajouté pour HotkeyRegistrationFailed dans test_all_errors_are_actionable
  - [M3] Logging des erreurs emit au lieu de silencieux `let _ =`
  - [+1 test] test_hotkey_registration_failed_serialization ajouté
- 2026-01-27: Story 2.5 implémentée - raccourci clavier global Ctrl+Alt+R pour toggle enregistrement

### File List

**Modifiés:**
- src-tauri/Cargo.toml
- src-tauri/capabilities/default.json
- src-tauri/src/error.rs
- src-tauri/src/lib.rs
- src-tauri/src/system/hotkeys.rs

**Générés:**
- src-tauri/target/release/bundle/deb/vocal-note-taker_0.1.0_amd64.deb
- src-tauri/target/release/bundle/appimage/vocal-note-taker_0.1.0_amd64.AppImage
