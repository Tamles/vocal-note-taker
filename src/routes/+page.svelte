<script lang="ts">
  /**
   * Vocal Note Taker - Main Page
   *
   * Main application layout with header, content area, and footer.
   * Displays version number and integrates error handling.
   *
   * @listens recording-started - Updates recordingState to 'recording'
   * @listens recording-stopped - Updates recordingState to 'transcribing'
   * @listens transcription-complete - Updates recordingState to 'idle'
   * @listens error - Displays error via errorStore
   * @listens keydown Ctrl+Q - Triggers graceful application quit
   */
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { recordingState, isRecording, isTranscribing, recordingDuration, audioData } from '../stores/recordingState';
  import { errorStore } from '../stores/errorStore';
  import { transcriptionProgress, transcriptionText, resetTranscription } from '../stores/transcriptionState';
  import ErrorNotification from '../components/ErrorNotification.svelte';
  import RecordButton from '../components/RecordButton.svelte';
  import Timer from '../components/Timer.svelte';
  import WaveformDisplay from '../components/WaveformDisplay.svelte';
  import ProgressBar from '../components/ProgressBar.svelte';
  import TranscriptionDisplay from '../components/TranscriptionDisplay.svelte';
  import CopyButton from '../components/CopyButton.svelte';
  import { toAppError } from '../lib/errorHelpers';

  let appVersion = '';
  let unlisteners: UnlistenFn[] = [];
  let isClosing = false;

  /**
   * Handles Ctrl+Q keyboard shortcut for graceful quit.
   * Fallback for when menu accelerator doesn't trigger.
   */
  function handleKeydown(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === 'q') {
      event.preventDefault();
      handleQuit();
    }
  }

  /**
   * Triggers graceful application shutdown via backend command.
   * Sets isClosing flag to show feedback during cleanup.
   */
  async function handleQuit() {
    if (isClosing) return; // Prevent multiple quit attempts
    isClosing = true;

    try {
      await invoke('request_quit');
    } catch (error) {
      console.error('Erreur lors de la fermeture:', error);
      isClosing = false;
    }
  }

  onMount(async () => {
    // Load version from backend
    try {
      appVersion = await invoke<string>('get_version');
    } catch {
      appVersion = '?';
    }

    // Setup IPC event listeners for backend events
    unlisteners.push(
      await listen('recording-started', () => recordingState.setRecording()),
      await listen<number>('recording-stopped', () => {
        // Transition to transcribing state - transcription is started by RecordButton
        recordingState.setTranscribing();
        resetTranscription();
      }),
      await listen<{ percent: number }>('transcription-progress', (event) => {
        transcriptionProgress.set(event.payload.percent);
      }),
      await listen<{ text: string }>('transcription-complete', (event) => {
        transcriptionText.set(event.payload.text);
        transcriptionProgress.set(100);
        recordingState.setIdle();
        // Reset recording state for next session
        recordingDuration.reset();
        audioData.clear();
      }),
      await listen<{ type: string; message: string }>('error', (event) => {
        errorStore.setError(toAppError(event.payload));
        // Reset to idle on error - AC #4: permet de relancer immédiatement
        recordingState.setIdle();
        transcriptionProgress.reset();
        // Keep audioData and recordingDuration for potential debugging
      }),
      await listen<number[]>('waveform-data', (event) => {
        audioData.append(event.payload);
      })
    );

    // Listen for Ctrl+Q keyboard shortcut
    document.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    unlisteners.forEach(unlisten => unlisten());
    document.removeEventListener('keydown', handleKeydown);
  });
</script>

<main class="app-container">
  <header>
    <h1>Vocal Note Taker</h1>
  </header>

  <section class="content">
    {#if isClosing}
      <p class="status-text closing">Fermeture en cours...</p>
    {:else}
      <!-- Bouton d'enregistrement avec indicateur REC -->
      <RecordButton />

      <!-- Timer d'enregistrement -->
      {#if $isRecording || $recordingDuration > 0}
        <Timer />
      {/if}

      <!-- Waveform - visible pendant l'enregistrement -->
      {#if $isRecording}
        <WaveformDisplay />
      {/if}

      <!-- Progress bar - visible pendant la transcription -->
      {#if $isTranscribing}
        <ProgressBar progress={$transcriptionProgress} />
      {/if}

      <!-- Transcription display - composant dédié -->
      {#if $transcriptionText && !$isRecording && !$isTranscribing}
        <TranscriptionDisplay />
        <CopyButton />
      {/if}

      <!-- Status text sous le bouton -->
      {#if $isRecording}
        <p class="status-text">Parlez maintenant...</p>
      {:else if $isTranscribing}
        <p class="status-text">Transcription en cours...</p>
      {:else if $transcriptionText}
        <p class="status-text">Transcription terminée</p>
      {:else}
        <p class="status-text">Cliquez pour enregistrer</p>
      {/if}
    {/if}
  </section>

  <footer>
    <span class="version">v{appVersion}</span>
  </footer>

  <ErrorNotification />
</main>

<style>
  :global(:root) {
    --color-bg: #1a1a2e;
    --color-bg-secondary: #16213e;
    --color-text: #eee;
    --color-text-muted: #888;
    --color-border: #333;
    --color-accent: #0f3460;
    --color-recording: #ef4444;
    --timer-font-size: 2rem;
    --font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background: var(--color-bg);
    color: var(--color-text);
    font-family: var(--font-family);
  }

  .app-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background: var(--color-bg);
    color: var(--color-text);
  }

  header {
    padding: 1rem;
    text-align: center;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-secondary);
  }

  header h1 {
    font-size: 1.5rem;
    margin: 0;
    color: #fff;
    font-weight: 600;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    gap: 1.5rem;
  }

  .status-text {
    color: var(--color-text-muted);
    font-size: 1.2rem;
    margin: 0;
  }

  .status-text.closing {
    color: #f0ad4e;
  }

  footer {
    padding: 0.5rem 1rem;
    text-align: right;
    border-top: 1px solid var(--color-border);
    background: var(--color-bg-secondary);
  }

  .version {
    font-size: 0.75rem;
    color: #666;
  }

  /* Responsive adjustments */
  @media (max-width: 480px) {
    header h1 {
      font-size: 1.25rem;
    }

    .status-text {
      font-size: 1rem;
    }

    .content {
      padding: 1rem;
    }
  }
</style>
