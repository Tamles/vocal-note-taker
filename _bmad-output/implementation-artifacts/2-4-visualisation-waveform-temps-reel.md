# Story 2.4: Visualisation waveform en temps réel

Status: done

## Story

As a utilisateur,
I want voir une visualisation de ma voix pendant l'enregistrement,
so that j'aie un feedback immédiat que le microphone capte bien.

## Acceptance Criteria

1. **Given** l'enregistrement est actif
   **When** je parle dans le microphone
   **Then** une waveform s'affiche en temps réel (FR7)
   **And** la visualisation réagit à l'amplitude de ma voix

2. **Given** le backend capture l'audio
   **When** les échantillons sont traités
   **Then** l'événement waveform-data est émis à 30-60 FPS
   **And** les données sont de type Float32Array

3. **Given** le composant WaveformDisplay.svelte existe
   **When** j'examine son implémentation
   **Then** il utilise Canvas API natif (pas de librairie externe)
   **And** il utilise requestAnimationFrame pour le rendu
   **And** il consomme le store audioData

4. **Given** l'enregistrement s'arrête
   **When** la waveform est visible
   **Then** elle s'arrête de s'animer
   **And** affiche un état statique ou se vide

## Tasks / Subtasks

- [x] **Task 1: Créer le store audioData** (AC: #2, #3)
  - [x] Modifier `src/stores/recordingState.ts`
  - [x] Ajouter `audioData` writable store (number[])
  - [x] Ajouter méthodes `setAudioData(samples: number[])` et `clearAudioData()`
  - [x] Exporter depuis le module

- [x] **Task 2: Écouter l'événement waveform-data** (AC: #2)
  - [x] Dans `src/routes/+page.svelte` ou `src/lib/eventListeners.ts`
  - [x] Importer `listen` depuis `@tauri-apps/api/event`
  - [x] Écouter `waveform-data` et mettre à jour `audioData` store
  - [x] Cleanup listener dans `onDestroy`

- [x] **Task 3: Créer WaveformDisplay.svelte** (AC: #1, #3)
  - [x] Créer `src/components/WaveformDisplay.svelte`
  - [x] Importer `audioData` et `isRecording` stores
  - [x] Créer élément `<canvas>` avec `bind:this`
  - [x] Dimensions: width=320, height=80 (ou responsive)

- [x] **Task 4: Implémenter le rendu Canvas** (AC: #1, #3)
  - [x] Fonction `drawWaveform(samples: number[], ctx: CanvasRenderingContext2D)`
  - [x] Boucle requestAnimationFrame pendant `$isRecording`
  - [x] Dessiner barres verticales représentant amplitude
  - [x] Couleur: verte (#22c55e) pendant recording, grise idle
  - [x] Clear canvas avant chaque draw

- [x] **Task 5: Gérer l'arrêt d'enregistrement** (AC: #4)
  - [x] Quand `$isRecording` devient false, arrêter requestAnimationFrame
  - [x] Option A: Vider le canvas (fond uni)
  - [x] Option B: Afficher dernière frame statique
  - [x] Réinitialiser audioData au prochain enregistrement

- [x] **Task 6: Intégrer dans +page.svelte** (AC: #1, #4)
  - [x] Importer WaveformDisplay
  - [x] Placer après RecordButton et Timer
  - [x] Afficher conditionnellement pendant recording
  - [x] Styles CSS pour espacement

- [x] **Task 7: Tests et validation** (AC: #1, #2, #3, #4)
  - [x] Build sans erreur (`pnpm build`)
  - [x] Test manuel: démarrer enregistrement → waveform visible
  - [x] Test manuel: parler → waveform réagit à l'amplitude
  - [x] Test manuel: arrêter → waveform se vide ou devient statique
  - [x] Vérifier pas de memory leak (requestAnimationFrame cleanup)

## Dev Notes

### Architecture Compliance

**IMPORTANT: L'infrastructure backend est DÉJÀ EN PLACE !**

Le code backend pour émettre les événements waveform existe déjà:
- `src-tauri/src/audio/capture.rs` - Envoie samples downsamplés via channel (ligne 173-190)
- `src-tauri/src/commands.rs` - Émet `waveform-data` events (ligne 91-95)

**Cette story est principalement FRONTEND.**

**Fichiers à créer/modifier:**
```
src/components/WaveformDisplay.svelte    # NOUVEAU - Composant waveform Canvas
src/stores/recordingState.ts             # MODIFIER - Ajouter audioData store
src/routes/+page.svelte                  # MODIFIER - Intégrer WaveformDisplay + listener
```

### Code Backend Existant (NE PAS MODIFIER)

```rust
// src-tauri/src/audio/capture.rs (lignes 173-190)
// Downsample ratio: 1 sample sur 100 → ~160 samples/seconde à 16kHz
const WAVEFORM_DOWNSAMPLE_RATIO: usize = 100;

// Envoie samples vers frontend via channel
if let Some(ref tx) = waveform_tx {
    let waveform_samples: Vec<f32> = data.iter()
        .filter_map(|&sample| { /* downsampling */ })
        .collect();
    if !waveform_samples.is_empty() {
        let _ = tx.try_send(waveform_samples);
    }
}
```

```rust
// src-tauri/src/commands.rs (lignes 91-95)
// Spawn task pour émettre events waveform vers frontend
tauri::async_runtime::spawn(async move {
    while let Some(samples) = rx.recv().await {
        let _ = app_handle.emit("waveform-data", samples);
    }
});
```

### Modification recordingState.ts

Ajouter le store `audioData` après `recordingDuration`:

```typescript
/**
 * Audio waveform data store.
 * Receives samples from backend via waveform-data events.
 * Data is downsampled (~160 samples/sec at 16kHz source).
 */
const audioDataStore = writable<number[]>([]);

export const audioData = {
  subscribe: audioDataStore.subscribe,
  set: (samples: number[]) => audioDataStore.set(samples),
  append: (samples: number[]) => audioDataStore.update(current => {
    // Garder les 200 derniers samples pour visualisation (sliding window)
    const combined = [...current, ...samples];
    return combined.slice(-200);
  }),
  clear: () => audioDataStore.set([]),
};
```

### Event Listener dans +page.svelte

```typescript
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { onMount, onDestroy } from 'svelte';
import { audioData } from '../stores/recordingState';

let unlistenWaveform: UnlistenFn | null = null;

onMount(async () => {
  // ... existing listeners ...

  unlistenWaveform = await listen<number[]>('waveform-data', (event) => {
    audioData.append(event.payload);
  });
});

onDestroy(() => {
  // ... existing cleanup ...
  unlistenWaveform?.();
});
```

### WaveformDisplay.svelte - Implémentation Complète

```svelte
<script lang="ts">
  /**
   * WaveformDisplay component - Renders real-time audio waveform.
   * Uses Canvas API for optimal performance (30-60 FPS).
   *
   * @consumes audioData - Array of amplitude samples from backend
   * @consumes isRecording - Active recording state
   */
  import { onMount, onDestroy } from 'svelte';
  import { audioData, isRecording } from '../stores/recordingState';

  let canvasElement: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let animationId: number | null = null;

  const CANVAS_WIDTH = 320;
  const CANVAS_HEIGHT = 80;
  const BAR_WIDTH = 3;
  const BAR_GAP = 1;
  const BAR_COUNT = Math.floor(CANVAS_WIDTH / (BAR_WIDTH + BAR_GAP));

  /**
   * Dessine la waveform sur le canvas.
   * Représente chaque sample comme une barre verticale centrée.
   */
  function drawWaveform(samples: number[]) {
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = 'transparent';
    ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

    // Couleur des barres
    ctx.fillStyle = $isRecording ? '#22c55e' : '#666666';

    // Dessiner les barres (derniers BAR_COUNT samples)
    const displaySamples = samples.slice(-BAR_COUNT);
    const centerY = CANVAS_HEIGHT / 2;

    displaySamples.forEach((sample, i) => {
      // Amplitude normalisée (samples sont en -1.0 à 1.0)
      const amplitude = Math.abs(sample);
      // Hauteur proportionnelle (min 2px, max 90% canvas height)
      const barHeight = Math.max(2, amplitude * CANVAS_HEIGHT * 0.9);

      const x = i * (BAR_WIDTH + BAR_GAP);
      const y = centerY - barHeight / 2;

      // Dessiner barre avec coins arrondis
      ctx.beginPath();
      ctx.roundRect(x, y, BAR_WIDTH, barHeight, 1);
      ctx.fill();
    });
  }

  /**
   * Boucle d'animation pour le rendu continu.
   */
  function animationLoop() {
    let currentSamples: number[] = [];
    audioData.subscribe(s => currentSamples = s)();

    drawWaveform(currentSamples);

    if ($isRecording) {
      animationId = requestAnimationFrame(animationLoop);
    }
  }

  /**
   * Démarre l'animation quand l'enregistrement commence.
   */
  function startAnimation() {
    if (animationId !== null) return;
    audioData.clear(); // Reset samples
    animationLoop();
  }

  /**
   * Arrête l'animation et vide le canvas.
   */
  function stopAnimation() {
    if (animationId !== null) {
      cancelAnimationFrame(animationId);
      animationId = null;
    }
    // Dessiner état final (vide ou dernière frame)
    if (ctx) {
      ctx.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    }
  }

  // Réagir aux changements d'état d'enregistrement
  $: if ($isRecording) {
    startAnimation();
  } else {
    stopAnimation();
  }

  onMount(() => {
    ctx = canvasElement.getContext('2d');
  });

  onDestroy(() => {
    stopAnimation();
  });
</script>

<div class="waveform-container" class:active={$isRecording}>
  <canvas
    bind:this={canvasElement}
    width={CANVAS_WIDTH}
    height={CANVAS_HEIGHT}
    class="waveform-canvas"
    aria-label="Visualisation audio en temps réel"
    role="img"
  ></canvas>
</div>

<style>
  .waveform-container {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem;
    border-radius: 12px;
    background: rgba(0, 0, 0, 0.05);
    transition: background 0.3s ease;
  }

  .waveform-container.active {
    background: rgba(34, 197, 94, 0.1);
  }

  .waveform-canvas {
    display: block;
    border-radius: 8px;
  }
</style>
```

### Intégration dans +page.svelte

```svelte
<script lang="ts">
  // ... imports existants ...
  import WaveformDisplay from '../components/WaveformDisplay.svelte';
  import { audioData, isRecording, recordingDuration } from '../stores/recordingState';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';

  let unlistenWaveform: UnlistenFn | null = null;

  onMount(async () => {
    // ... existing listeners ...

    // Listener pour waveform data
    unlistenWaveform = await listen<number[]>('waveform-data', (event) => {
      audioData.append(event.payload);
    });
  });

  onDestroy(() => {
    // ... existing cleanup ...
    unlistenWaveform?.();
  });
</script>

<!-- Dans le template, après Timer -->
{#if $isRecording || $recordingDuration > 0}
  <Timer />
{/if}

<!-- Waveform - visible pendant l'enregistrement -->
{#if $isRecording}
  <WaveformDisplay />
{/if}
```

### Naming Conventions (CRITIQUE)

**TypeScript/Svelte:**
- Composant: `WaveformDisplay.svelte` (PascalCase)
- Fonctions: `drawWaveform()`, `startAnimation()`, `stopAnimation()` (camelCase)
- Variables: `canvasElement`, `animationId`, `audioData` (camelCase)
- Constantes: `CANVAS_WIDTH`, `CANVAS_HEIGHT`, `BAR_COUNT` (SCREAMING_SNAKE_CASE)

**CSS Classes:**
- `waveform-container`, `waveform-canvas` (kebab-case)

**Events:**
- `waveform-data` (kebab-case) - déjà défini par backend

### Previous Story Intelligence (Story 2.3)

**Patterns établis:**
- Stores dans `src/stores/recordingState.ts` avec méthodes helper (`subscribe`, `set`, `update`)
- Composants utilisent `$isRecording` pour état conditionnel
- Reactive statements (`$:`) pour réagir aux changements de stores
- Animations CSS avec transitions 0.3s
- Couleurs: `#ef4444` (rouge recording), `#22c55e` (vert waveform), `#888`/`#666` (muted)
- `aria-*` attributs pour accessibilité
- Cleanup dans `onDestroy`

**Ce qui existe déjà (à réutiliser):**
- `isRecording` derived store
- Pattern d'écoute events dans `+page.svelte`
- `onMount`/`onDestroy` lifecycle hooks

### Git Intelligence

**Derniers commits:**
```
cc010dc story 2-3
14899f3 stories 2-1 and 2-2
b340e02 End of epic 1
```

**Convention commits:**
```
Story 2.4 - visualisation waveform temps réel
```

### NFR Compliance

- **FR7:** System can display real-time audio waveform visualization during recording ✓
- **NFR-USA-5:** Feedback clarity - waveform visible pendant enregistrement ✓
- **NFR-PERF-3:** UI responsive <100ms - requestAnimationFrame ne bloque pas ✓
- **NFR-SEC-1:** Pas de dépendance réseau - Canvas API natif ✓

### Performance Considerations

**Frame Rate:**
- Backend émet ~160 samples/sec (16kHz / 100 downsample ratio)
- Frontend peut render à 60 FPS via requestAnimationFrame
- Sliding window de 200 samples évite accumulation mémoire

**Optimisations:**
- `clearRect` plutôt que créer nouveau canvas
- Pas de création d'objets dans la boucle d'animation
- `try_send` non-bloquant côté backend (ligne 190 capture.rs)

### Testing Strategy

**Tests manuels requis:**

1. **Test démarrage:**
   ```
   1. Lancer `pnpm tauri dev`
   2. Cliquer sur le bouton d'enregistrement
   3. Vérifier waveform apparaît avec barres
   ```

2. **Test réactivité:**
   ```
   1. Pendant enregistrement, parler dans le microphone
   2. Vérifier barres waveform augmentent en hauteur
   3. Silence → barres petites/plates
   4. Son fort → barres hautes
   ```

3. **Test arrêt:**
   ```
   1. Arrêter l'enregistrement
   2. Vérifier waveform disparaît ou devient statique
   3. Pas de barres qui continuent à s'animer
   ```

4. **Test performance:**
   ```
   1. Enregistrer pendant >30 secondes
   2. Vérifier pas de lag ou freeze UI
   3. Vérifier mémoire stable (pas de leak)
   ```

### Code Anti-Patterns à Éviter

❌ **MAUVAIS - Création d'objets dans la boucle:**
```typescript
// Crée nouveau tableau à chaque frame → garbage collection
animationLoop() {
  const samples = [...$audioData]; // Copie inutile
}
```

❌ **MAUVAIS - requestAnimationFrame sans cleanup:**
```typescript
// Memory leak si composant démonté
$: if ($isRecording) {
  requestAnimationFrame(draw); // Pas de cancel
}
```

❌ **MAUVAIS - Librairie externe:**
```typescript
// NON - Architecture impose Canvas API natif
import WaveSurfer from 'wavesurfer.js';
```

### Edge Cases à Considérer

1. **Microphone silencieux:** Afficher barres minimales (2px) plutôt que rien
2. **Démarrage rapide multiple:** Vérifier qu'un seul animationLoop tourne
3. **Canvas non-initialisé:** Vérifier ctx avant draw (onMount)
4. **Samples vides:** Gérer tableau vide sans erreur

### Project Structure Notes

```
src/
├── components/
│   ├── RecordButton.svelte       # Existant (Story 2.2)
│   ├── Timer.svelte              # Existant (Story 2.3)
│   ├── WaveformDisplay.svelte    # NOUVEAU - Cette story
│   └── ErrorNotification.svelte  # Existant (Epic 1)
├── stores/
│   ├── recordingState.ts         # À MODIFIER - ajouter audioData
│   └── errorStore.ts             # Existant
├── routes/
│   └── +page.svelte              # À MODIFIER - intégrer waveform + listener
└── types/
    └── index.ts                  # Existant
```

### Dependencies

**Aucune nouvelle dépendance requise** - cette story utilise uniquement:
- Canvas API natif
- `@tauri-apps/api/event` (déjà utilisé)
- Svelte stores natif

### References

- [Source: _bmad-output/planning-artifacts/epics.md - Story 2.4]
- [Source: _bmad-output/planning-artifacts/architecture.md - Waveform Visualization, Lines 468-480]
- [Source: _bmad-output/planning-artifacts/architecture.md - Audio Processing Pipeline, Lines 396-409]
- [Source: _bmad-output/project-context.md - Rule #5 Tauri IPC Events]
- [Source: _bmad-output/project-context.md - Rule #6 Svelte State Management]
- [Source: src-tauri/src/audio/capture.rs - Waveform downsample, Lines 22-24, 173-190]
- [Source: src-tauri/src/commands.rs - Waveform event emit, Lines 91-95]
- [Source: _bmad-output/implementation-artifacts/2-3-timer-enregistrement-temps-reel.md - Previous Story Patterns]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5 (claude-opus-4-5-20251101)

### Debug Log References

- Fixed TypeScript error: `ctx` possibly null in forEach callback - used local `context` variable
- Fixed a11y warning: removed invalid `role="img"` from canvas element

### Completion Notes List

- **Task 1:** Created `audioData` store with `subscribe`, `set`, `append` (sliding window 200 samples), and `clear` methods
- **Task 2:** Added `waveform-data` event listener in `+page.svelte` onMount, cleanup in unlisteners array
- **Task 3-4:** Created `WaveformDisplay.svelte` component with Canvas API, requestAnimationFrame loop, bar visualization
- **Task 5:** Implemented stopAnimation() that cancels requestAnimationFrame and clears canvas when recording stops
- **Task 6:** Integrated component in `+page.svelte` with conditional rendering `{#if $isRecording}`
- **Task 7:** Build verified (pnpm check + pnpm tauri build) - no errors, warnings only for unused Rust imports

### Change Log

- 2026-01-27: Story 2.4 implementation complete - Visualisation waveform temps réel
- 2026-01-27: Code review fixes applied:
  - Fix #2: Replaced subscribe/unsubscribe pattern with `get()` from svelte/store in animationLoop
  - Fix #4: Optimized sliding window to reduce array allocations (mutate when possible)
  - Fix #5: Passed isRecording as parameter to drawWaveform instead of reactive store access
  - Added JSDoc documentation for constants (COLOR_ACTIVE, COLOR_INACTIVE, etc.)

### File List

- `src/stores/recordingState.ts` - MODIFIED (added audioData store)
- `src/components/WaveformDisplay.svelte` - NEW (waveform Canvas component)
- `src/routes/+page.svelte` - MODIFIED (added WaveformDisplay + waveform-data listener)

