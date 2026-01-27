<script lang="ts">
  /**
   * Composant Timer - Affiche le temps d'enregistrement écoulé.
   *
   * @consumes isRecording - État d'enregistrement actif
   * @consumes recordingDuration - Durée en secondes
   * @updates recordingDuration - Incrémente via setInterval
   */
  import { onDestroy } from 'svelte';
  import { isRecording, recordingDuration } from '../stores/recordingState';

  let intervalId: ReturnType<typeof setInterval> | null = null;

  /**
   * Formate les secondes en SS ou MM:SS selon la durée.
   * @param seconds - Nombre de secondes
   * @returns Chaîne formatée (ex: "05" pour <60s, "01:23" pour >=60s)
   */
  function formatDuration(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    if (mins === 0) {
      return secs.toString().padStart(2, '0');
    }
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  /**
   * Démarre le timer quand l'enregistrement commence.
   */
  function startTimer() {
    // Clear any existing interval to prevent memory leak
    stopTimer();

    // Réinitialiser au démarrage
    recordingDuration.reset();

    // Démarrer l'incrémentation chaque seconde
    intervalId = setInterval(() => {
      recordingDuration.increment();
    }, 1000);
  }

  /**
   * Arrête le timer quand l'enregistrement se termine.
   */
  function stopTimer() {
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }

  // Reactive: réagir aux changements de isRecording
  $: if ($isRecording) {
    startTimer();
  } else {
    stopTimer();
  }

  // Cleanup au démontage du composant
  onDestroy(() => {
    stopTimer();
  });
</script>

<div class="timer-container" class:active={$isRecording} aria-live="polite" role="timer">
  <span class="timer-display">{formatDuration($recordingDuration)}</span>
</div>

<style>
  .timer-container {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    transition: all 0.3s ease;
  }

  .timer-container.active {
    background: rgba(239, 68, 68, 0.1);
  }

  .timer-display {
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Code', monospace;
    font-size: var(--timer-font-size, 2rem);
    font-weight: 600;
    letter-spacing: 0.05em;
    color: var(--color-text-muted, #888);
    transition: color 0.3s ease;
  }

  /* Animation subtile du chiffre qui change */
  @keyframes digitPulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.8;
    }
  }

  .timer-container.active .timer-display {
    color: var(--color-recording, #ef4444);
    animation: digitPulse 1s ease-in-out infinite;
  }
</style>
