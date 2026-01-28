<script lang="ts">
  /**
   * Composant RecordButton - Toggle enregistrement avec feedback visuel.
   *
   * @consumes recordingState - État actuel (idle/recording/transcribing)
   * @invokes start_recording - Démarre la capture audio
   * @invokes stop_recording - Arrête la capture audio, retourne le chemin WAV
   */
  import { invoke } from '@tauri-apps/api/core';
  import { recordingState, isRecording, isTranscribing } from '../stores/recordingState';
  import { errorStore } from '../stores/errorStore';
  import { toAppError } from '../lib/errorHelpers';

  let isLoading = false;

  /**
   * Toggle l'état d'enregistrement via commandes IPC backend.
   * - idle → start_recording → recording
   * - recording → stop_recording → transcribing → start_transcription
   */
  async function handleClick() {
    if (isLoading || $isTranscribing) return;

    isLoading = true;

    try {
      if ($isRecording) {
        // Stop recording - returns the WAV file path
        const wavPath = await invoke<string>('stop_recording');

        // Start transcription with the WAV file
        // This returns immediately, results come via events
        await invoke('start_transcription', { audioPath: wavPath });
      } else {
        // Start recording - backend emits recording-started event
        await invoke('start_recording');
      }
    } catch (error) {
      console.error('Recording/transcription error:', error);
      errorStore.setError(toAppError(error));
      // Reset to idle on error
      recordingState.setIdle();
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="record-button-container">
  <!-- Indicateur REC - Visible uniquement pendant l'enregistrement -->
  {#if $isRecording}
    <div class="rec-indicator" aria-live="polite" role="status">
      <span class="rec-dot" aria-hidden="true"></span>
      <span class="rec-text">REC</span>
    </div>
  {/if}

  <!-- Bouton principal d'enregistrement -->
  <button
    class="record-button"
    class:recording={$isRecording}
    class:transcribing={$isTranscribing}
    class:loading={isLoading}
    on:click={handleClick}
    disabled={$isTranscribing || isLoading}
    aria-label={$isRecording ? 'Arrêter l\'enregistrement' : 'Démarrer l\'enregistrement'}
    aria-pressed={$isRecording}
  >
    <span class="button-icon">
      {#if $isTranscribing}
        <!-- Spinner de chargement pendant la transcription -->
        <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" stroke-opacity="0.3" />
          <path d="M12 2a10 10 0 0 1 10 10" />
        </svg>
      {:else if $isRecording}
        <!-- Icône stop (carré) pendant l'enregistrement -->
        <svg viewBox="0 0 24 24" fill="currentColor">
          <rect x="6" y="6" width="12" height="12" rx="2" />
        </svg>
      {:else}
        <!-- Icône microphone en état idle -->
        <svg viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 14c1.66 0 3-1.34 3-3V5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3z"/>
          <path d="M17 11c0 2.76-2.24 5-5 5s-5-2.24-5-5H5c0 3.53 2.61 6.43 6 6.92V21h2v-3.08c3.39-.49 6-3.39 6-6.92h-2z"/>
        </svg>
      {/if}
    </span>
  </button>
</div>

<style>
  .record-button-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  /* REC Indicator */
  .rec-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.75rem;
    background: rgba(239, 68, 68, 0.2);
    border-radius: 4px;
    animation: fadeIn 0.3s ease-out;
  }

  .rec-dot {
    width: 12px;
    height: 12px;
    background-color: #ef4444;
    border-radius: 50%;
    animation: pulse 1s ease-in-out infinite;
  }

  .rec-text {
    color: #ef4444;
    font-weight: 700;
    font-size: 0.875rem;
    letter-spacing: 0.1em;
  }

  /* Main Button */
  .record-button {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    border: 4px solid #444;
    background: linear-gradient(135deg, #2d2d2d 0%, #1a1a1a 100%);
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
  }

  .record-button:hover:not(:disabled) {
    border-color: #666;
    transform: scale(1.05);
  }

  .record-button:focus {
    outline: none;
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.4), 0 4px 15px rgba(0, 0, 0, 0.3);
  }

  .record-button:active:not(:disabled) {
    transform: scale(0.95);
  }

  /* Recording State */
  .record-button.recording {
    border-color: #ef4444;
    background: linear-gradient(135deg, #3d1a1a 0%, #1a0a0a 100%);
    animation: recordingPulse 2s ease-in-out infinite;
  }

  .record-button.recording .button-icon {
    color: #ef4444;
  }

  /* Transcribing State */
  .record-button.transcribing {
    border-color: #666;
    background: linear-gradient(135deg, #2d2d2d 0%, #1a1a1a 100%);
    cursor: not-allowed;
    opacity: 0.7;
  }

  .record-button:disabled {
    cursor: not-allowed;
  }

  /* État loading - feedback visuel pendant l'appel IPC */
  .record-button.loading {
    opacity: 0.8;
    pointer-events: none;
  }

  .record-button.loading .button-icon {
    animation: loadingPulse 0.8s ease-in-out infinite;
  }

  /* Button Icon */
  .button-icon {
    width: 32px;
    height: 32px;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .button-icon svg {
    width: 100%;
    height: 100%;
  }

  /* Spinner Animation */
  .spinner {
    animation: spin 1s linear infinite;
  }

  /* Animations */
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(0.9);
    }
  }

  @keyframes recordingPulse {
    0%, 100% {
      box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.4), 0 4px 15px rgba(0, 0, 0, 0.3);
    }
    50% {
      box-shadow: 0 0 0 10px rgba(239, 68, 68, 0), 0 4px 15px rgba(0, 0, 0, 0.3);
    }
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes loadingPulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
