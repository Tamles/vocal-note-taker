# Story 3.3: Gestion des erreurs de transcription

Status: done

## Story

As a utilisateur,
I want être informé clairement si la transcription échoue,
so that je puisse comprendre le problème et réessayer.

## Acceptance Criteria

1. **Given** le fichier audio est corrompu ou invalide
   **When** la transcription est tentée
   **Then** AppError::TranscriptionFailed est retourné (FR13)
   **And** un message d'erreur clair est affiché
   **And** l'application reste fonctionnelle

2. **Given** le modèle whisper n'est pas chargé correctement
   **When** la transcription est tentée
   **Then** une erreur explicite indique le problème du modèle
   **And** des instructions de résolution sont fournies

3. **Given** une erreur de transcription se produit
   **When** l'erreur est propagée au frontend
   **Then** le composant ErrorNotification.svelte l'affiche
   **And** l'utilisateur peut relancer un enregistrement

4. **Given** la transcription échoue
   **When** l'état est mis à jour
   **Then** recordingState revient à 'idle'
   **And** l'interface permet de recommencer

## Tasks / Subtasks

- [x] **Task 1: Ajouter validation audio avant transcription** (AC: #1)
  - [x] Dans whisper.rs, valider le format WAV (header, sample rate, channels)
  - [x] Créer AppError::InvalidAudioFormat(String) pour fichiers audio invalides
  - [x] Retourner message clair si fichier corrompu/vide/format incorrect
  - [x] Ajouter test pour validation audio

- [x] **Task 2: Améliorer les messages d'erreur de chargement modèle** (AC: #2)
  - [x] Enrichir AppError::ModelNotFound avec instructions détaillées
  - [x] Enrichir AppError::ModelLoadFailed avec diagnostic (mémoire, corruption, permissions)
  - [x] Vérifier que les instructions de download-models.sh sont incluses

- [x] **Task 3: Compléter les types TypeScript** (AC: #3)
  - [x] Ajouter 'ModelNotFound', 'ModelLoadFailed', 'InvalidAudioFormat' dans AppErrorType
  - [x] Mettre à jour les tests de sérialisation

- [x] **Task 4: Enrichir ErrorNotification pour types d'erreur** (AC: #3)
  - [x] Afficher icône différente selon le type d'erreur (modèle vs audio vs transcription)
  - [x] Optionnel: Ajouter bouton d'action contextuel (ex: "Télécharger modèle")

- [x] **Task 5: Vérifier la récupération d'état** (AC: #4)
  - [x] Confirmer que recordingState passe à 'idle' sur toute erreur
  - [x] Confirmer que transcriptionProgress est réinitialisé
  - [x] Confirmer que l'utilisateur peut immédiatement relancer un enregistrement

- [x] **Task 6: Ajouter tests d'intégration erreurs** (AC: #1, #2, #3, #4)
  - [x] Test: fichier audio vide → erreur claire
  - [x] Test: fichier audio corrompu → erreur claire
  - [x] Test: modèle absent → instructions téléchargement
  - [x] Test: récupération après erreur → état idle, interface fonctionnelle

## Dev Notes

### Architecture Compliance

**Cette story touche BACKEND (Rust) et FRONTEND (TypeScript)**

**Fichiers à modifier:**
```
BACKEND (Rust):
src-tauri/src/error.rs                    # MODIFIER - Ajouter InvalidAudioFormat
src-tauri/src/transcription/whisper.rs    # MODIFIER - Validation audio pré-transcription
src-tauri/src/commands.rs                 # VÉRIFIER - Gestion erreurs complète

FRONTEND (TypeScript/Svelte):
src/types/index.ts                        # MODIFIER - Ajouter nouveaux types erreur
src/components/ErrorNotification.svelte   # OPTIONNEL - Améliorer affichage par type
src/routes/+page.svelte                   # VÉRIFIER - Recovery path complet
```

### CRITIQUE: Privacy-First (NFR-SEC-1)

**Zero Network Calls - Même en cas d'erreur:**
- ❌ **INTERDIT**: Envoi de diagnostic/télémétrie
- ❌ **INTERDIT**: Téléchargement automatique du modèle
- ✅ **OBLIGATOIRE**: Toutes les erreurs gérées localement
- ✅ **OBLIGATOIRE**: Instructions claires pour résolution manuelle

### Ce qui existe déjà (Stories 3-1 et 3-2)

**AppError variants existants:**
```rust
// src-tauri/src/error.rs
#[error("Transcription échouée: {0}. Réessayez l'enregistrement.")]
TranscriptionFailed(String),

#[error("Modèle Whisper non trouvé. {0}")]
ModelNotFound(String),

#[error("Échec du chargement du modèle Whisper: {0}")]
ModelLoadFailed(String),
```

**Gestion erreurs dans commands.rs (start_transcription):**
```rust
// Erreur modèle non trouvé
Err(e) => {
    eprintln!("Failed to get model path: {:?}", e);
    let _ = app_clone.emit("error", e);
    return;
}

// Erreur transcription
Err(e) => {
    eprintln!("Transcription failed: {:?}", e);
    let _ = app_clone.emit("error", e);
}
```

**Frontend error handling (+page.svelte):**
```typescript
await listen<{ type: string; message: string }>('error', (event) => {
    errorStore.setError(toAppError(event.payload));
    recordingState.setIdle();  // ✓ Récupération état OK
});
```

**ErrorNotification.svelte:** Affiche `$errorStore.message` avec auto-dismiss 5s.

### Pattern d'implémentation: Validation Audio

```rust
// src-tauri/src/transcription/whisper.rs

/// Valide le format d'un fichier WAV avant transcription.
///
/// # Errors
/// - `InvalidAudioFormat` si header WAV invalide, samples vides, ou format incorrect
fn validate_wav_file(path: &Path) -> Result<(), AppError> {
    let reader = hound::WavReader::open(path)
        .map_err(|e| AppError::InvalidAudioFormat(format!(
            "Fichier audio invalide ou corrompu: {}",
            e
        )))?;

    let spec = reader.spec();

    // Vérifier channels (doit être mono)
    if spec.channels != 1 {
        return Err(AppError::InvalidAudioFormat(format!(
            "Audio doit être mono (1 canal), reçu: {} canaux",
            spec.channels
        )));
    }

    // Vérifier sample rate (16kHz attendu)
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
            "Fichier audio vide - aucun échantillon détecté".to_string()
        ));
    }

    Ok(())
}

/// Transcrit un fichier audio WAV avec validation préalable.
pub fn transcribe_audio(
    model: &WhisperModel,
    audio_path: &Path,
) -> Result<String, AppError> {
    // 1. Valider le fichier audio AVANT de lire les samples
    validate_wav_file(audio_path)?;

    // 2. Lire les samples (existant)
    let samples = read_wav_samples(audio_path)?;

    // 3. Reste de la transcription (existant)
    // ...
}
```

### Nouvelle variante AppError: InvalidAudioFormat

```rust
// Ajouter dans error.rs

#[error("Format audio invalide: {0}. Réenregistrez.")]
InvalidAudioFormat(String),
```

```rust
// Ajouter dans serialize match
AppError::InvalidAudioFormat(_) => "InvalidAudioFormat",
```

```rust
// Ajouter dans test_all_errors_are_actionable
AppError::InvalidAudioFormat("test".to_string()),
```

### Types TypeScript à compléter

```typescript
// src/types/index.ts

export type AppErrorType =
  | 'MicrophoneAccessDenied'
  | 'MicrophoneNotFound'
  | 'TranscriptionFailed'
  | 'RecordingInterrupted'
  | 'ConfigurationError'
  | 'ClipboardError'
  | 'IoError'
  | 'HotkeyRegistrationFailed'
  | 'ModelNotFound'         // AJOUTER
  | 'ModelLoadFailed'       // AJOUTER
  | 'InvalidAudioFormat';   // AJOUTER
```

### Amélioration optionnelle ErrorNotification

```svelte
<!-- src/components/ErrorNotification.svelte -->
<script lang="ts">
  import { errorStore } from '../stores/errorStore';
  import { fly } from 'svelte/transition';

  // Icône selon le type d'erreur
  function getErrorIcon(type: string): string {
    switch (type) {
      case 'ModelNotFound':
      case 'ModelLoadFailed':
        return '📦'; // Problème modèle
      case 'InvalidAudioFormat':
      case 'TranscriptionFailed':
        return '🎤'; // Problème audio
      case 'MicrophoneAccessDenied':
      case 'MicrophoneNotFound':
        return '🔇'; // Problème microphone
      default:
        return '⚠️'; // Erreur générique
    }
  }
</script>

{#if $errorStore}
  <div class="error-notification" transition:fly={{ y: -20, duration: 300 }}>
    <div class="error-content">
      <span class="error-icon">{getErrorIcon($errorStore.type)}</span>
      <span class="error-message">{$errorStore.message}</span>
    </div>
    <button class="close-button" on:click={() => errorStore.clearError()}>✕</button>
  </div>
{/if}
```

### Naming Conventions (CRITIQUE)

**Rust:**
- Variante: `InvalidAudioFormat(String)` (PascalCase)
- Fonction: `validate_wav_file()` (snake_case)

**TypeScript:**
- Type: `'InvalidAudioFormat'` (string literal, PascalCase)

### Messages d'erreur actionnables (FR47)

Tous les messages d'erreur DOIVENT inclure une action suggérée:

| Erreur | Message | Action |
|--------|---------|--------|
| InvalidAudioFormat | "Format audio invalide: {détail}" | "Réenregistrez." |
| TranscriptionFailed | "Transcription échouée: {détail}" | "Réessayez l'enregistrement." |
| ModelNotFound | "Modèle Whisper non trouvé." | "Exécutez: ./scripts/download-models.sh" |
| ModelLoadFailed | "Échec du chargement du modèle: {détail}" | Diagnostic contextualisé |

### Scénarios d'erreur à couvrir

1. **Fichier audio vide** → Enregistrement trop court ou micro non capturé
2. **Fichier audio corrompu** → Problème écriture WAV
3. **Mauvais format audio** → Channels ou sample rate incorrect (normalement impossible avec notre pipeline)
4. **Modèle absent** → User n'a pas téléchargé le modèle
5. **Modèle corrompu** → Téléchargement interrompu ou fichier endommagé
6. **Mémoire insuffisante** → Modèle ~3GB ne peut pas être chargé

### Previous Story Intelligence (Stories 3-1 et 3-2)

**Patterns établis à réutiliser:**
- Result<T, AppError> pour toutes les fonctions
- Émission événement "error" vers frontend avec payload sérialisé
- Auto-dismiss ErrorNotification après 5 secondes
- Récupération état `idle` sur erreur dans +page.svelte

**Fichiers créés/modifiés dans les stories précédentes:**
- `src-tauri/src/error.rs` - AppError avec thiserror
- `src-tauri/src/transcription/whisper.rs` - transcribe_audio, read_wav_samples
- `src-tauri/src/commands.rs` - start_transcription avec gestion erreurs
- `src/stores/errorStore.ts` - setError/clearError
- `src/components/ErrorNotification.svelte` - Affichage erreurs
- `src/routes/+page.svelte` - Listener événement error

### Git Intelligence

**Derniers commits:**
```
91de8e3 Story 3-2
f1d7f39 Story 3-1
cd4297e Story 2-4
```

**Convention commit:**
```
Story 3-3 - gestion erreurs transcription
```

### NFR Compliance

- **FR13:** System can handle transcription errors gracefully with clear error messages ✓
- **FR47:** System can provide clear actionable error messages to user ✓
- **FR48:** System can continue operating after non-critical errors ✓
- **NFR-REL-4:** Application must recover gracefully from non-critical errors ✓

### Edge Cases à Considérer

1. **Erreur pendant chargement modèle** → Déjà géré dans start_transcription
2. **Erreur pendant transcription** → Déjà géré, émet événement error
3. **Double erreur** → errorStore remplace l'erreur précédente (comportement actuel)
4. **Erreur puis succès** → État réinitialisé, transcription suivante fonctionne

### Tests à implémenter

```rust
// src-tauri/src/transcription/whisper.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_wav_empty_file() {
        // Créer fichier WAV vide temporaire
        let temp_dir = std::env::temp_dir();
        let empty_wav = temp_dir.join("empty_test.wav");
        // Écrire header WAV avec 0 samples
        // ...
        let result = validate_wav_file(&empty_wav);
        assert!(matches!(result, Err(AppError::InvalidAudioFormat(_))));
    }

    #[test]
    fn test_validate_wav_nonexistent_file() {
        let result = validate_wav_file(Path::new("/nonexistent/file.wav"));
        assert!(matches!(result, Err(AppError::InvalidAudioFormat(_))));
    }

    #[test]
    fn test_error_messages_are_actionable() {
        let err = AppError::InvalidAudioFormat("test".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Réenregistrez"), "Should suggest action");
    }
}
```

### Project Structure Notes

**Alignement avec structure définie:**
```
src-tauri/src/
├── error.rs                    # À MODIFIER - InvalidAudioFormat
├── transcription/
│   └── whisper.rs              # À MODIFIER - validate_wav_file

src/
├── types/
│   └── index.ts                # À MODIFIER - Nouveaux types erreur
├── components/
│   └── ErrorNotification.svelte # OPTIONNEL - Amélioration
└── routes/
    └── +page.svelte            # VÉRIFIER - Recovery OK
```

### Scope et Boundaries

**INCLUS dans cette story:**
- Validation audio pré-transcription
- Nouvelle variante erreur InvalidAudioFormat
- Complétion types TypeScript
- Vérification recovery path
- Tests unitaires erreurs

**EXCLUS de cette story:**
- Amélioration UI ErrorNotification (optionnel, peut être différé)
- Retry automatique (non requis par PRD)
- Logging persistant des erreurs (non requis)
- Analytics/télémétrie (INTERDIT par NFR-SEC-1)

### References

- [Source: _bmad-output/planning-artifacts/epics.md - Story 3.3]
- [Source: _bmad-output/project-context.md - Rule #2 Error Handling Strict]
- [Source: _bmad-output/project-context.md - Rule #1 Privacy-First Architecture]
- [Source: src-tauri/src/error.rs - AppError pattern existant]
- [Source: src-tauri/src/transcription/whisper.rs - transcribe_audio existant]
- [Source: src-tauri/src/commands.rs - start_transcription error handling]
- [Source: src/components/ErrorNotification.svelte - UI erreurs existante]
- [Source: src/stores/errorStore.ts - Store erreurs existant]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- Tests: 44 passed, 0 failed
- svelte-check: 0 errors, 0 warnings

### Completion Notes List

1. **Task 1**: Ajout de `validate_wav_file()` dans whisper.rs avec validation complète (header WAV, channels mono, sample rate 16kHz, fichier non vide). Création de `AppError::InvalidAudioFormat(String)` avec message actionnable.

2. **Task 2**: Amélioration de `ModelLoadFailed` avec diagnostic contextuel (mémoire insuffisante, fichier corrompu) et instructions download-models.sh dans le message d'erreur.

3. **Task 3**: Ajout des types `ModelNotFound`, `ModelLoadFailed`, `InvalidAudioFormat`, `HotkeyRegistrationFailed` dans `AppErrorType` TypeScript.

4. **Task 4**: Amélioration de `ErrorNotification.svelte` avec fonction `getErrorIcon()` retournant une icône spécifique par type d'erreur (📦 modèle, 🎤 audio, 🔇 microphone, etc.).

5. **Task 5**: Correction du recovery path dans +page.svelte - ajout de `transcriptionProgress.reset()` sur erreur pour permettre relance immédiate.

6. **Task 6**: Tests d'intégration ajoutés couvrant tous les scénarios d'erreur (fichier vide, corrompu, mauvais format, modèle absent).

### Change Log

- 2026-01-28: Story 3-3 implémentée - Gestion erreurs transcription complète
- 2026-01-29: Code review - Corrections M2 (validation redondante), M3 (test renforcé), M4 (commentaire)

### File List

- src-tauri/src/error.rs (modifié)
- src-tauri/src/transcription/whisper.rs (modifié)
- src/types/index.ts (modifié)
- src/components/ErrorNotification.svelte (modifié)
- src/routes/+page.svelte (modifié)
- _bmad-output/implementation-artifacts/sprint-status.yaml (modifié)
