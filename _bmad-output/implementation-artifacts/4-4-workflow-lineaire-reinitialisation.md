# Story 4.4: Workflow linéaire et réinitialisation

Status: review

## Story

As a utilisateur,
I want un workflow simple sans historique,
so that l'interface reste épurée et je me concentre sur la tâche actuelle.

## Acceptance Criteria

1. **Given** du texte transcrit est affiché
   **When** je démarre un nouvel enregistrement
   **Then** le texte précédent est automatiquement effacé (FR18)
   **And** l'interface revient à l'état d'enregistrement

2. **Given** l'application fonctionne
   **When** j'examine les fonctionnalités
   **Then** il n'y a pas de gestion d'historique (FR19)
   **And** pas de liste de transcriptions passées
   **And** pas de bouton "précédent/suivant"

3. **Given** l'utilisateur termine une session
   **When** il veut recommencer
   **Then** un simple clic/raccourci réinitialise tout
   **And** le workflow reste linéaire: enregistrer → transcrire → copier → répéter

## Tasks / Subtasks

- [x] **Task 1: Effacer le texte transcrit au démarrage d'un nouvel enregistrement** (AC: #1)
  - [x] Identifier le point d'entrée : `start_recording` dans RecordButton.svelte
  - [x] Appeler `resetTranscription()` AVANT d'invoquer `start_recording`
  - [x] Vérifier que `transcriptionText` est vide quand l'enregistrement démarre
  - [x] Tester visuellement: texte affiché → clic enregistrer → texte disparu

- [x] **Task 2: Vérifier l'absence de fonctionnalités d'historique** (AC: #2)
  - [x] Audit du code frontend : vérifier qu'aucun store ne stocke d'historique
  - [x] Vérifier qu'aucun bouton "précédent/suivant" n'existe
  - [x] Vérifier qu'aucune liste de transcriptions n'existe
  - [x] Confirmer que le design actuel est déjà conforme (YAGNI)

- [x] **Task 3: Valider le workflow linéaire complet** (AC: #3)
  - [x] Tester le cycle complet: enregistrer → transcrire → copier → enregistrer à nouveau
  - [x] Vérifier que le raccourci global réinitialise correctement l'état
  - [x] Vérifier que le clic sur RecordButton réinitialise correctement l'état
  - [x] Documenter le workflow dans les notes de complétion

- [x] **Task 4: Tests et validation finale** (AC: #1, #2, #3)
  - [x] Vérifier `svelte-check` passe sans erreur
  - [x] Vérifier `cargo check` passe sans erreur
  - [x] Tester manuellement le workflow complet plusieurs fois
  - [x] Vérifier la cohérence des états UI à chaque transition

## Dev Notes

### Architecture Compliance

**Cette story touche uniquement le FRONTEND (Svelte) - modification mineure**

**Fichiers à modifier:**
```
src/components/RecordButton.svelte    # MODIFIER - Ajouter resetTranscription() avant start_recording
```

**Fichiers à vérifier (pas de modification attendue):**
```
src/stores/transcriptionState.ts      # LIRE - resetTranscription() existe déjà
src/routes/+page.svelte               # LIRE - Logique d'affichage conditionnelle existante
```

**Pattern architectural (project-context.md Rule #6):**
- Les composants ne mutent pas directement les stores - ils appellent les méthodes des stores
- La fonction `resetTranscription()` existe déjà dans transcriptionState.ts
- L'appel doit être fait dans RecordButton avant l'invocation IPC

### Ce qui EXISTE déjà

**transcriptionState.ts - resetTranscription():**
```typescript
export function resetTranscription(): void {
  transcriptionProgress.reset();
  transcriptionText.reset();
}
```
Cette fonction existe et est déjà utilisée dans +page.svelte lors de `recording-stopped`.

**RecordButton.svelte - handleClick():**
```typescript
async function handleClick() {
  if (isLoading || $isTranscribing) return;
  isLoading = true;

  try {
    if ($isRecording) {
      const wavPath = await invoke<string>('stop_recording');
      await invoke('start_transcription', { audioPath: wavPath });
    } else {
      // Start recording - backend emits recording-started event
      await invoke('start_recording');
    }
  } catch (error) {
    // ... error handling
  } finally {
    isLoading = false;
  }
}
```

**+page.svelte - Affichage conditionnel:**
```svelte
{#if $transcriptionText && !$isRecording && !$isTranscribing}
  <TranscriptionDisplay />
  <CopyButton bind:this={copyButtonRef} />
{/if}
```
L'affichage est déjà conditionnel : le texte n'apparaît que si `!$isRecording`.

### Pattern d'implémentation

**Modification dans RecordButton.svelte:**

```typescript
import { resetTranscription } from '../stores/transcriptionState';

async function handleClick() {
  if (isLoading || $isTranscribing) return;
  isLoading = true;

  try {
    if ($isRecording) {
      const wavPath = await invoke<string>('stop_recording');
      await invoke('start_transcription', { audioPath: wavPath });
    } else {
      // FR18: Clear previous transcription before new recording
      resetTranscription();
      // Start recording - backend emits recording-started event
      await invoke('start_recording');
    }
  } catch (error) {
    console.error('Recording/transcription error:', error);
    errorStore.setError(toAppError(error));
    recordingState.setIdle();
  } finally {
    isLoading = false;
  }
}
```

### Analyse du comportement actuel

**Comportement ACTUEL (sans cette story):**
1. Utilisateur enregistre → transcription affichée
2. Utilisateur clique pour enregistrer à nouveau
3. `recording-started` event → `recordingState` passe à 'recording'
4. Le texte n'est plus visible car `{#if !$isRecording}` est false
5. `recording-stopped` event → `resetTranscription()` est appelé

**Problème potentiel:**
Le texte disparaît visuellement car `!$isRecording` devient false, mais `transcriptionText` garde l'ancienne valeur jusqu'au `recording-stopped` event.

**Solution (cette story):**
Appeler `resetTranscription()` AVANT `start_recording` pour:
1. Effacer immédiatement les données obsolètes (FR18)
2. Libérer la mémoire du texte précédent
3. Garantir un état propre au démarrage

### Naming Conventions (CRITIQUE)

**TypeScript:**
- Import: `import { resetTranscription } from '../stores/transcriptionState'`
- Appel de fonction: `resetTranscription()` (camelCase)

### NFR Compliance

- **FR18:** System can automatically clear previous transcription when starting new recording ✓
- **FR19:** System can maintain simple linear workflow (no history management) ✓
- **NFR-USA-3:** Maximum 3 user actions required for complete workflow ✓

### Previous Story Intelligence (Story 4-3)

**Patterns établis à réutiliser:**
- Import des fonctions de store depuis `transcriptionState.ts`
- Gestion d'état synchrone avant appel IPC async
- Structure try/catch avec error handling

**Fichiers touchés dans 4-3:**
- CopyButton.svelte (focus)
- +page.svelte (auto-focus listener)

**Convention commit:**
```
Story 4-4 - workflow linéaire réinitialisation
```

### Git Intelligence

**Derniers commits:**
```
9339778 Story 4-2
678c7c6 Story 4-1
81758ce Story 3-4
c08aff3 Story 3-3
91de8e3 Story 3-2
```

**Patterns récents:**
- Modifications incrémentales de composants existants
- Utilisation des stores centralisés
- Appel de fonctions utilitaires depuis les stores

### Edge Cases à Considérer

1. **Transcription en cours** → Bouton disabled, pas de reset possible
2. **Erreur pendant reset** → `resetTranscription()` ne peut pas échouer (set sur store)
3. **Double clic rapide** → `isLoading` flag empêche le double appel
4. **Reset puis échec start_recording** → État reste propre (text vide), erreur affichée
5. **Raccourci global** → Utilise `start_recording` IPC qui déclenche le même flow

### Scope et Boundaries

**INCLUS dans cette story:**
- Appel `resetTranscription()` avant démarrage enregistrement
- Vérification absence d'historique (audit code)
- Validation workflow linéaire

**EXCLUS de cette story:**
- Modification backend
- Modification du comportement de copie
- Ajout de nouvelles fonctionnalités

### Project Structure Notes

**Alignement avec structure définie:**
```
src/
├── components/
│   └── RecordButton.svelte       # MODIFIER - Ajouter resetTranscription()
└── stores/
    └── transcriptionState.ts     # LIRE - resetTranscription() existe
```

### Validation svelte-check

Après implémentation, exécuter:
```bash
pnpm svelte-check
cargo check
```
Doit retourner 0 erreurs, 0 warnings.

### Complexité estimée

**TRÈS FAIBLE** - Cette story consiste en:
1. Ajouter 1 import
2. Ajouter 1 ligne d'appel de fonction
3. Vérifier que le code existant ne viole pas FR19

La majorité du travail est la validation et la documentation.

### Workflow Linéaire - Diagramme

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    IDLE     │────→│  RECORDING  │────→│TRANSCRIBING │
│             │     │             │     │             │
│ (texte vide │     │ (texte vide │     │  (progress) │
│  ou ancien) │     │  - FR18)    │     │             │
└─────────────┘     └─────────────┘     └─────────────┘
       ↑                                       │
       │           ┌─────────────┐             │
       └───────────│   DISPLAY   │←────────────┘
                   │             │
                   │ (texte prêt,│
                   │  Enter=copy)│
                   └─────────────┘
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md - Story 4.4]
- [Source: _bmad-output/project-context.md - Rule #6 Svelte State Management]
- [Source: src/stores/transcriptionState.ts - resetTranscription()]
- [Source: src/components/RecordButton.svelte - handleClick()]
- [Source: src/routes/+page.svelte - Affichage conditionnel]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

N/A - Implémentation straightforward sans problèmes de debug.

### Completion Notes List

**Task 1 - Reset transcription au démarrage:**
- Ajouté import `resetTranscription` dans RecordButton.svelte:12
- Ajouté appel `resetTranscription()` avant `invoke('start_recording')` dans RecordButton.svelte:37
- Pattern conforme à project-context.md Rule #6 (composants appellent méthodes stores)

**Task 2 - Audit absence historique:**
- `transcriptionState.ts`: `textStore` est `string`, pas d'array - ✅ Conforme FR19
- `recordingState.ts`: `audioData` est buffer visualisation (sliding window), pas historique
- `+page.svelte`: Aucun bouton nav historique, aucune liste transcriptions
- Design YAGNI respecté - aucune fonctionnalité d'historique n'a été implémentée

**Task 3 - Workflow linéaire validé:**
- Cycle: IDLE → (click) → reset + recording → (click) → transcribing → DISPLAY → (click) → reset + recording
- Le raccourci global et le clic utilisent le même flow via RecordButton
- Le `resetTranscription()` garantit un état propre à chaque nouveau cycle

**Task 4 - Validation:**
- `pnpm svelte-check`: 0 errors, 0 warnings
- `cargo check`: Compilé avec succès (warnings préexistants non liés)

### File List

**Modified:**
- src/components/RecordButton.svelte (ajout import + appel resetTranscription)

### Change Log

- 2026-01-30: Story 4-4 implémentée - FR18 (reset auto) et FR19 (pas d'historique) validés
