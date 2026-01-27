<script lang="ts">
  /**
   * WaveformDisplay component - Renders real-time audio waveform.
   * Uses Canvas API for optimal performance (30-60 FPS).
   *
   * @consumes audioData - Array of amplitude samples from backend
   * @consumes isRecording - Active recording state
   */
  import { onMount, onDestroy } from 'svelte';
  import { get } from 'svelte/store';
  import { audioData, isRecording } from '../stores/recordingState';

  let canvasElement: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let animationId: number | null = null;

  /** Canvas width in pixels - matches typical compact UI width */
  const CANVAS_WIDTH = 320;
  /** Canvas height in pixels - compact but visible waveform */
  const CANVAS_HEIGHT = 80;
  /** Width of each amplitude bar in pixels */
  const BAR_WIDTH = 3;
  /** Gap between bars in pixels */
  const BAR_GAP = 1;
  /** Number of bars that fit in the canvas */
  const BAR_COUNT = Math.floor(CANVAS_WIDTH / (BAR_WIDTH + BAR_GAP));
  /** Active recording color (green) */
  const COLOR_ACTIVE = '#22c55e';
  /** Inactive color (gray) */
  const COLOR_INACTIVE = '#666666';

  /**
   * Dessine la waveform sur le canvas.
   * Représente chaque sample comme une barre verticale centrée.
   * @param samples - Array of amplitude values (-1.0 to 1.0)
   * @param isActive - Whether recording is active (determines bar color)
   */
  function drawWaveform(samples: number[], isActive: boolean) {
    const context = ctx;
    if (!context) return;

    // Clear canvas
    context.clearRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

    // Couleur des barres selon état d'enregistrement
    context.fillStyle = isActive ? COLOR_ACTIVE : COLOR_INACTIVE;

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
      context.beginPath();
      context.roundRect(x, y, BAR_WIDTH, barHeight, 1);
      context.fill();
    });
  }

  /**
   * Boucle d'animation pour le rendu continu.
   * Utilise get() pour lecture synchrone sans subscription overhead.
   */
  function animationLoop() {
    // Lecture synchrone des stores - pas de subscription/unsubscription par frame
    const currentSamples = get(audioData);
    const recording = get(isRecording);

    drawWaveform(currentSamples, recording);

    if (recording) {
      animationId = requestAnimationFrame(animationLoop);
    }
  }

  /**
   * Démarre l'animation quand l'enregistrement commence.
   */
  function startAnimation() {
    if (animationId !== null) return;
    audioData.clear();
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
    // Vider le canvas à l'arrêt
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
