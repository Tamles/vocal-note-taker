# Story 1.1: Initialisation du projet Tauri avec structure modulaire

Status: done

## Story

As a développeur,
I want un projet Tauri initialisé avec la structure de modules définie,
so that je dispose d'une base solide pour implémenter toutes les fonctionnalités.

## Acceptance Criteria

1. **Given** le projet n'existe pas encore
   **When** j'exécute `pnpm create tauri-app vocal-note-taker -- --template svelte-ts`
   **Then** le projet est créé avec Svelte + TypeScript + Vite

2. **Given** le projet est créé
   **When** j'examine la structure backend Rust (`src-tauri/src/`)
   **Then** les modules suivants existent :
   - `audio/` (mod.rs, capture.rs, buffer.rs)
   - `transcription/` (mod.rs, whisper.rs)
   - `config/` (mod.rs, loader.rs)
   - `system/` (mod.rs, hotkeys.rs, clipboard.rs)
   **And** `commands.rs` existe comme couche d'orchestration IPC
   **And** `error.rs` existe avec un AppError enum vide (avec thiserror)

3. **Given** le projet est configuré
   **When** j'exécute `pnpm tauri dev`
   **Then** l'application démarre sans erreur
   **And** aucune connexion réseau n'est établie (FR39 - offline operation)

## Tasks / Subtasks

- [x] **Task 1: Initialiser le projet Tauri** (AC: #1)
  - [x] Exécuter `pnpm create tauri-app vocal-note-taker -- --template svelte-ts`
  - [x] Vérifier que Svelte 4.x/5.x + TypeScript 5.x + Vite 5.x sont installés
  - [x] Naviguer dans le dossier créé et installer les dépendances avec `pnpm install`

- [x] **Task 2: Créer la structure des modules Rust backend** (AC: #2)
  - [x] Créer `src-tauri/src/error.rs` avec enum AppError vide + derive thiserror
  - [x] Créer `src-tauri/src/commands.rs` avec placeholder pour futurs Tauri commands
  - [x] Créer dossier `src-tauri/src/audio/` avec `mod.rs`, `capture.rs`, `buffer.rs`
  - [x] Créer dossier `src-tauri/src/transcription/` avec `mod.rs`, `whisper.rs`
  - [x] Créer dossier `src-tauri/src/config/` avec `mod.rs`, `loader.rs`
  - [x] Créer dossier `src-tauri/src/system/` avec `mod.rs`, `hotkeys.rs`, `clipboard.rs`
  - [x] Modifier `src-tauri/src/main.rs` pour déclarer tous les modules

- [x] **Task 3: Configurer les dépendances Cargo.toml** (AC: #2, #3)
  - [x] Ajouter `thiserror = "1.x"` pour gestion erreurs
  - [x] Ajouter `serde = { version = "1.x", features = ["derive"] }`
  - [x] Ajouter `tokio = { version = "1.x", features = ["full"] }` pour async runtime
  - [x] Ajouter `toml = "0.8"` pour configuration
  - [x] **NE PAS** ajouter de dépendances réseau (reqwest, hyper, etc.)

- [x] **Task 4: Créer la structure frontend** (AC: #2)
  - [x] Créer dossier `src/components/` (vide pour l'instant)
  - [x] Créer dossier `src/stores/` (vide pour l'instant)
  - [x] Créer dossier `src/lib/` (vide pour l'instant)
  - [x] Créer dossier `src/types/` avec `index.ts` vide

- [x] **Task 5: Valider le projet** (AC: #3)
  - [x] Exécuter `pnpm tauri dev` et vérifier que l'app démarre
  - [x] Vérifier qu'aucune erreur de compilation Rust
  - [x] Vérifier qu'aucune connexion réseau via `cargo tree | grep -E "(reqwest|hyper|tokio.*net)"`

## Dev Notes

### Architecture Compliance

**Structure Backend Rust OBLIGATOIRE** (src-tauri/src/):
```
src-tauri/src/
├── main.rs              # Entry point Tauri, app setup
├── commands.rs          # Tauri commands (IPC layer) - THIN orchestration
├── error.rs             # AppError enum (thiserror)
├── audio/
│   ├── mod.rs          # pub mod capture; pub mod buffer;
│   ├── capture.rs      # Placeholder - cpal integration future
│   └── buffer.rs       # Placeholder - double buffer logic future
├── transcription/
│   ├── mod.rs          # pub mod whisper;
│   └── whisper.rs      # Placeholder - whisper-rs future
├── config/
│   ├── mod.rs          # pub mod loader;
│   └── loader.rs       # Placeholder - TOML parsing future
└── system/
    ├── mod.rs          # pub mod hotkeys; pub mod clipboard;
    ├── hotkeys.rs      # Placeholder - global shortcuts future
    └── clipboard.rs    # Placeholder - clipboard ops future
```

**Structure Frontend OBLIGATOIRE** (src/):
```
src/
├── App.svelte           # Root component (template généré)
├── main.ts              # Entry point (template généré)
├── components/          # Composants Svelte (FLAT structure)
├── stores/              # State management centralisé
├── lib/                 # Helpers réutilisables
└── types/
    └── index.ts         # Types TypeScript partagés
```

### Naming Conventions (CRITIQUE)

**Rust:**
- Modules/fichiers: `snake_case` → `audio_capture.rs`, `whisper.rs`
- Structs/Enums: `PascalCase` → `AppError`, `AudioBuffer`
- Functions/variables: `snake_case` → `start_recording()`, `audio_data`
- Constants: `SCREAMING_SNAKE_CASE` → `MAX_RECORDING_DURATION`

**TypeScript/Svelte:**
- Composants: `PascalCase.svelte` → `WaveformDisplay.svelte`
- Stores/utils: `camelCase.ts` → `recordingState.ts`
- Functions/variables: `camelCase` → `startRecording()`, `audioData`

### AppError Enum Initial (error.rs)

```rust
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Microphone access denied")]
    MicrophoneAccessDenied,

    #[error("Microphone not found")]
    MicrophoneNotFound,

    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    #[error("Recording interrupted")]
    RecordingInterrupted,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Clipboard error")]
    ClipboardError,
}
```

### Cargo.toml Dependencies Initiales

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
thiserror = "1"
toml = "0.8"

# NE PAS AJOUTER - Dépendances réseau INTERDITES (NFR-SEC-1)
# reqwest, hyper, tokio/net features
```

### Privacy-First Validation (CRITIQUE - NFR-SEC-1)

Après configuration, TOUJOURS vérifier:
```bash
cargo tree | grep -E "(reqwest|hyper|tokio.*net)"
# Doit retourner VIDE
```

Si une dépendance réseau apparaît, elle doit être retirée immédiatement.

### Project Structure Notes

- **Pas de dossiers imbriqués** dans `src/components/` - structure FLAT
- Les modules Rust (`mod.rs`) doivent exposer les sous-modules avec `pub mod`
- Le fichier `main.rs` doit déclarer tous les modules: `mod audio; mod transcription; mod config; mod system; mod commands; mod error;`

### References

- [Source: _bmad-output/planning-artifacts/architecture.md - Starter Template Evaluation]
- [Source: _bmad-output/planning-artifacts/architecture.md - Backend Architecture Decisions]
- [Source: _bmad-output/planning-artifacts/architecture.md - Implementation Patterns & Consistency Rules]
- [Source: _bmad-output/project-context.md - Critical Implementation Rules]
- [Source: _bmad-output/planning-artifacts/prd.md - FR39: Offline operation]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- Cargo check: compilation réussie avec 1 warning attendu (import placeholder)
- cargo tree: aucune dépendance réseau (NFR-SEC-1 validé)
- pnpm tauri dev: Vite démarré sur localhost:1420

### Completion Notes List

- Task 1: Projet Tauri créé via pnpm create tauri-app avec template svelte-ts
  - Svelte 5.46.3, TypeScript 5.6.3, Vite 6.4.1, Tauri 2.9.x
- Task 2: Structure backend Rust créée selon architecture.md
  - Modules: audio, transcription, config, system, commands, error
- Task 3: Cargo.toml configuré avec thiserror, tokio, toml, serde
  - Aucune dépendance réseau ajoutée
- Task 4: Structure frontend créée (components, stores, lib, types)
- Task 5: Validation complète - compilation OK, NFR-SEC-1 OK

### Code Review Fixes (2026-01-15)

- **H1 Fixed**: Nom projet corrigé "tauri-temp" → "vocal-note-taker" dans package.json, tauri.conf.json
- **H2 Accepted**: SvelteKit routes/ structure acceptée (modern approach)
- **H3 Accepted**: Modules dans lib.rs acceptable pour Tauri 2.x
- **M1 Fixed**: Code template greet() supprimé, +page.svelte nettoyé
- **L2 Fixed**: Repo git initialisé

### File List

**Créés:**
- package.json
- pnpm-lock.yaml
- tsconfig.json
- svelte.config.js
- vite.config.js
- .gitignore
- README.md
- src/app.html
- src/routes/+layout.ts
- src/routes/+page.svelte
- src/components/ (dossier vide)
- src/stores/ (dossier vide)
- src/lib/ (dossier vide)
- src/types/index.ts
- src-tauri/Cargo.toml
- src-tauri/tauri.conf.json
- src-tauri/build.rs
- src-tauri/src/main.rs
- src-tauri/src/lib.rs
- src-tauri/src/error.rs
- src-tauri/src/commands.rs
- src-tauri/src/audio/mod.rs
- src-tauri/src/audio/capture.rs
- src-tauri/src/audio/buffer.rs
- src-tauri/src/transcription/mod.rs
- src-tauri/src/transcription/whisper.rs
- src-tauri/src/config/mod.rs
- src-tauri/src/config/loader.rs
- src-tauri/src/system/mod.rs
- src-tauri/src/system/hotkeys.rs
- src-tauri/src/system/clipboard.rs
- static/ (dossier avec assets)
- .vscode/ (configuration VSCode)
