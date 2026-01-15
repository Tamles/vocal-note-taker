---
project_name: 'vocal-note-taker'
user_name: 'Tamles'
date: '2026-01-13'
sections_completed: ['technology_stack', 'critical_rules', 'language_rules', 'framework_rules', 'testing_rules', 'structure_rules', 'anti_patterns']
existing_patterns_found: 9
status: 'complete'
rule_count: 12
optimized_for_llm: true
---

# Project Context for AI Agents - vocal-note-taker

_Ce fichier contient les règles critiques et patterns que les agents IA DOIVENT suivre lors de l'implémentation de code. Focus sur détails non-évidents que les agents pourraient manquer._

---

## Technology Stack & Versions

**Core Stack:**
- **Tauri:** 2.x (desktop framework, webview natif)
- **Rust:** stable avec Tokio 1.x (async runtime)
- **Svelte:** 4.x ou 5.x + TypeScript 5.x (frontend réactif)
- **Vite:** 5.x (build tool avec HMR)

**Audio & Transcription:**
- **whisper-rs:** 0.x (⚠️ Vérifier version stable latest avant Story 7)
- **cpal:** 0.15 (audio capture cross-platform)
- **hound:** 3.x (WAV encoding)

**Tauri Plugins (tous 2.x):**
- tauri-plugin-global-shortcut
- tauri-plugin-clipboard
- tauri-plugin-notification

**Configuration:**
- **toml:** 0.8 + serde (config parsing)
- **thiserror:** 1.x (error types)

**⚠️ Version Critique:** Avant d'implémenter Story 7 (whisper integration), RECHERCHER version stable whisper-rs sur crates.io et TESTER compatibility avec ggml-large.bin.

---

## Critical Implementation Rules

### 1. Privacy-First Architecture (NFR-SEC-1 - CRITIQUE)

**ABSOLU - Zero Network Calls:**
- ❌ **INTERDIT** : Aucune dépendance crate avec feature network (reqwest, hyper, tokio/net, etc.)
- ❌ **INTERDIT** : Fallback cloud pour transcription
- ✅ **OBLIGATOIRE** : Whisper.cpp 100% local uniquement
- ✅ **OBLIGATOIRE** : Cleanup immédiat fichiers audio temporaires après transcription

**Validation:**
```bash
# Vérifier aucune dep réseau dans Cargo.toml
cargo tree | grep -E "(reqwest|hyper|tokio.*net)"
# Doit retourner vide
```

**Cleanup Pattern:**
```rust
// Après transcription
let temp_path = Path::new(&audio_path);
fs::remove_file(temp_path)?; // Cleanup immédiat
```

### 2. Rust - Error Handling Strict

**TOUJOURS Result<T, AppError> - JAMAIS panic:**

✅ **CORRECT:**
```rust
pub fn load_config(path: &Path) -> Result<AppConfig, AppError> {
    let content = fs::read_to_string(path)
        .map_err(|e| AppError::ConfigError(format!("Cannot read: {}", e)))?;
    toml::from_str(&content)
        .map_err(|e| AppError::ConfigError(format!("Invalid TOML: {}", e)))
}
```

❌ **INTERDIT:**
```rust
let config = load_config(&path).unwrap(); // PANIC si erreur!
let data = get_data().expect("data"); // Pas de context pourquoi impossible
```

✅ **Exception OK:**
```rust
let data = get_data().expect("data must exist after validation"); // Context clair
```

**Règle:** Si `.unwrap()` ou `.expect()` utilisé, DOIT avoir comment expliquant pourquoi panic impossible.

### 3. Rust - Naming Conventions

**Module & Fichiers:**
- `snake_case` pour modules : `audio_capture.rs`, `whisper.rs`
- Pas de traits ou tirets dans noms fichiers Rust

**Fonctions & Variables:**
- `snake_case` : `start_recording()`, `audio_data`, `is_recording`

**Structs & Enums:**
- `PascalCase` : `AppError`, `AudioBuffer`, `AppConfig`

**Constantes:**
- `SCREAMING_SNAKE_CASE` : `MAX_RECORDING_DURATION`, `DEFAULT_SAMPLE_RATE`

### 4. TypeScript/Svelte - Naming Conventions

**Fichiers Composants:**
- `PascalCase.svelte` : `WaveformDisplay.svelte`, `RecordButton.svelte`

**Fichiers Stores/Utils:**
- `camelCase.ts` : `recordingState.ts`, `audioHelpers.ts`

**Fonctions & Variables:**
- `camelCase` : `startRecording()`, `audioData`, `isRecording`

**Interfaces & Types:**
- `PascalCase` : `AppConfig`, `WaveformData`, `TranscriptionResult`

### 5. Tauri IPC - Commands & Events

**Commands (Frontend → Backend):**
- Naming: `snake_case`
- Signature: TOUJOURS `Result<T, AppError>`

**MVP Commands (EXACTEMENT 4):**
```rust
#[tauri::command]
fn start_recording() -> Result<(), AppError>

#[tauri::command]
fn stop_recording() -> Result<String, AppError> // Retourne WAV path

#[tauri::command]
fn load_config() -> Result<AppConfig, AppError>

#[tauri::command]
fn copy_to_clipboard(text: String) -> Result<(), AppError>
```

⚠️ **NOTE MVP:** `save_config()` N'EST PAS inclus dans MVP. Config éditée manuellement par user dans TOML. Hot reload déféré post-MVP.

**Events (Backend → Frontend):**
- Naming: `kebab-case`
- Payload: Structure directe sans wrapper

**Events Types:**
```typescript
"waveform-data"            : { samples: number[] }
"transcription-progress"   : { percent: number }     // 0-100
"transcription-complete"   : { text: string }
"error"                    : { message: string, code?: string }
"recording-started"        : {}
"recording-stopped"        : { duration: number }
```

**Emission (Rust):**
```rust
app.emit_all("waveform-data", WaveformPayload { samples: vec![...] })?;
```

**Listening (TypeScript):**
```typescript
listen<WaveformPayload>('waveform-data', (event) => {
  audioData.set(event.payload.samples);
});
```

### 6. Svelte - State Management

**Stores Pattern:**
```typescript
// Writable pour state mutable
export const recordingState = writable<'idle' | 'recording' | 'transcribing'>('idle');

// Derived pour computed values
export const isRecording = derived(recordingState, $s => $s === 'recording');
```

**Update Patterns:**
```typescript
// set() pour remplacement complet
recordingState.set('recording');

// update() pour transformation
recordingDuration.update(n => n + 1);
```

**❌ Anti-Pattern:**
```typescript
// Redondant! Utiliser derived au lieu
export const isRecording = writable<boolean>(false);
```

### 7. Async Patterns - Tokio

**Long-Running Tasks:**
```rust
#[tauri::command]
async fn start_transcription(app_handle: AppHandle, path: String) -> Result<(), AppError> {
    tokio::spawn(async move {
        // Processing lourd dans task séparée
        match transcribe_audio(&path).await {
            Ok(text) => {
                app_handle.emit_all("transcription-complete",
                    TranscriptionPayload { text }).ok();
            },
            Err(e) => {
                app_handle.emit_all("error", ErrorPayload {
                    message: format!("Transcription failed: {}", e),
                    code: Some("TRANSCRIPTION_FAILED".to_string())
                }).ok();
            }
        }
    });

    Ok(()) // Retour immédiat, processing async
}
```

**Channels pour Streaming:**
```rust
let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<f32>>(100);

// Producer
tokio::spawn(async move {
    while let Some(samples) = get_audio_samples() {
        tx.send(samples).await.ok();
    }
});

// Consumer
tokio::spawn(async move {
    while let Some(samples) = rx.recv().await {
        app_handle.emit_all("waveform-data", WaveformPayload { samples }).ok();
    }
});
```

### 8. Project Structure - Organisation Stricte

**Backend Rust (`src-tauri/src/`):**
```
src-tauri/src/
├── main.rs              # Entry point, Tauri setup
├── commands.rs          # IPC commands (THIN orchestration)
├── error.rs             # AppError enum
├── audio/
│   ├── mod.rs
│   ├── capture.rs       # cpal integration
│   └── buffer.rs        # Double buffer logic
├── transcription/
│   ├── mod.rs
│   └── whisper.rs       # whisper-rs integration
├── config/
│   ├── mod.rs
│   └── loader.rs        # TOML parsing
└── system/
    ├── mod.rs
    ├── hotkeys.rs
    └── clipboard.rs
```

**⚠️ RÈGLE:** Ne PAS créer nouveaux dossiers sans justification architecturale. Utiliser structure définie.

**Frontend Svelte (`src/`):**
```
src/
├── App.svelte           # Root component
├── main.ts              # Entry, event listeners setup
├── components/          # FLAT structure (pas de nested folders)
│   ├── RecordButton.svelte
│   ├── WaveformDisplay.svelte
│   ├── Timer.svelte
│   ├── TranscriptionDisplay.svelte
│   ├── ProgressBar.svelte
│   └── ErrorNotification.svelte
├── stores/              # State management centralisé
│   ├── recordingState.ts
│   ├── transcriptionState.ts
│   ├── configStore.ts
│   └── errorStore.ts
├── lib/                 # Helpers réutilisables
│   ├── audioHelpers.ts
│   ├── formatters.ts
│   └── constants.ts
└── types/
    └── index.ts
```

### 9. Testing - Critical Paths (≥70% Coverage Backend)

**Tests Co-localisés:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_with_defaults() {
        // Test config loading avec defaults
    }
}
```

**Integration Tests (`src-tauri/tests/`):**
```rust
// tests/audio_tests.rs
#[test]
fn test_full_recording_flow() {
    // start_recording → samples buffered → stop_recording → WAV exists
}
```

**Critical Paths à Tester:**
1. Audio capture → WAV file created
2. Config loading → defaults fallback si fichier manquant
3. Error propagation → Result<T, AppError> fonctionne
4. Transcription flow → progress events émis
5. Cleanup → temp files deleted

### 10. Anti-Patterns INTERDITS

**❌ Mixing Naming Conventions:**
```rust
// MAUVAIS - camelCase en Rust
fn startRecording() -> Result<(), AppError> { }
```

**❌ Polling au lieu d'Events:**
```typescript
// MAUVAIS - polling backend
setInterval(async () => {
  const progress = await invoke('get_transcription_progress');
}, 100);

// CORRECT - event listener
listen<ProgressPayload>('transcription-progress', (event) => {
  transcriptionProgress.set(event.payload.percent);
});
```

**❌ Stores Redondants:**
```typescript
// MAUVAIS
export const recordingState = writable('idle');
export const isRecording = writable(false); // Redondant!

// CORRECT
export const recordingState = writable('idle');
export const isRecording = derived(recordingState, $s => $s === 'recording');
```

**❌ State Mutation Direct dans Composants:**
```svelte
<!-- MAUVAIS -->
<script>
  function handleClick() {
    recordingState.set('recording'); // Composant ne doit pas muter stores
  }
</script>

<!-- CORRECT -->
<script>
  async function handleClick() {
    await invoke('start_recording'); // Backend émet event → listener update store
  }
</script>
```

### 11. Documentation Requirements

**Fonctions Publiques:**
```rust
/// Charge la configuration depuis le fichier TOML.
///
/// # Errors
/// Retourne `AppError::ConfigError` si fichier introuvable ou TOML invalide.
pub fn load_config(path: &Path) -> Result<AppConfig, AppError> {
    // ...
}
```

**Composants Complexes:**
```typescript
/**
 * WaveformDisplay component - Renders real-time audio waveform
 *
 * @listens waveform-data - Receives audio samples from backend
 * @updates Canvas rendering at 30-60 FPS
 */
```

### 12. Performance Constraints (NFRs)

**Targets à Respecter:**
- Workflow end-to-end : <15s
- Transcription 60s audio : <30s
- UI responsive : <100ms (async Tokio garantit)
- RAM idle : <200MB (profiling déféré runtime)
- Waveform FPS : 30-60 (throttling events si besoin)

**Pattern Double Buffer Audio:**
```rust
// Buffer 1: WAV file complet (qualité transcription)
// Buffer 2: Samples downsampled 1/100 pour waveform (performance)
```

---

## Références Architecturales

**Document Complet:** `_bmad-output/planning-artifacts/architecture.md`

**Sections Clés:**
- Implementation Patterns & Consistency Rules (p.676+)
- Project Structure & Boundaries (p.1200+)
- Architecture Validation Results (p.1800+)

**En Cas de Doute:** Consulter architecture.md pour décisions détaillées et rationale.

---

---

## Usage Guidelines

**For AI Agents:**

- **READ THIS FILE FIRST** : Avant d'implémenter toute feature, lire ce fichier complètement
- **FOLLOW ALL RULES** : Suivre TOUTES les règles exactement comme documentées, sans exception
- **WHEN IN DOUBT** : Préférer l'option la plus restrictive et consulter architecture.md
- **UPDATE IF NEEDED** : Si nouveaux patterns émergent pendant implémentation, documenter ici
- **ZERO TOLERANCE** : Privacy-first (NFR-SEC-1) et error handling strict sont NON-NÉGOCIABLES

**For Humans:**

- **Keep Lean** : Ce fichier doit rester focalisé sur règles critiques que les agents IA pourraient manquer
- **Update Technology Changes** : Mettre à jour immédiatement quand versions ou stack technologique changent
- **Review Quarterly** : Revoir tous les 3 mois pour supprimer règles devenues évidentes
- **Remove Obvious Rules** : Supprimer règles que tous les agents suivent naturellement (garder seulement non-évident)
- **Validate Compliance** : Vérifier périodiquement que code généré respecte toutes règles

**Maintenance Schedule:**

- **Après chaque Story** : Vérifier si nouveaux patterns doivent être ajoutés
- **Mensuel** : Review rapide cohérence règles vs code produit
- **Trimestriel** : Cleanup complet (supprimer obsolète, optimiser wording)
- **Changement Stack** : Update immédiat si nouvelles deps ou versions majeures

---

**Dernière Mise à Jour:** 2026-01-13
**Version Architecture:** Complete (8 steps)
**Status:** Ready for Implementation
**Rule Count:** 12 sections critiques
**Optimized for LLM:** Yes
