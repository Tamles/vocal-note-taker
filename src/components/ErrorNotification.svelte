<script lang="ts">
  /**
   * ErrorNotification component - Displays error messages from backend.
   *
   * @consumes errorStore - Subscribes to error state
   * Features:
   * - Auto-dismisses after 5 seconds
   * - Manual close button
   * - Slide-in/out animation
   * - Red/orange alert styling
   * - Icon differentiation based on error type
   */
  import { errorStore } from '../stores/errorStore';
  import { fly } from 'svelte/transition';

  /**
   * Returns an appropriate icon based on the error type.
   * Groups errors by category for visual distinction.
   */
  function getErrorIcon(type: string | undefined): string {
    if (!type) return '⚠️';

    switch (type) {
      case 'ModelNotFound':
      case 'ModelLoadFailed':
        return '📦'; // Problème modèle
      case 'InvalidAudioFormat':
      case 'TranscriptionFailed':
        return '🎤'; // Problème audio/transcription
      case 'MicrophoneAccessDenied':
      case 'MicrophoneNotFound':
        return '🔇'; // Problème microphone
      case 'ConfigurationError':
        return '⚙️'; // Problème configuration
      case 'IoError':
        return '💾'; // Problème système fichiers
      case 'ClipboardError':
        return '📋'; // Problème presse-papiers
      case 'HotkeyRegistrationFailed':
        return '⌨️'; // Problème raccourci clavier
      default:
        return '⚠️'; // Erreur générique
    }
  }
</script>

{#if $errorStore}
  <div
    class="error-notification"
    transition:fly={{ y: -20, duration: 300 }}
  >
    <div class="error-content">
      <span class="error-icon">{getErrorIcon($errorStore.type)}</span>
      <span class="error-message">{$errorStore.message}</span>
    </div>
    <button
      class="close-button"
      on:click={() => errorStore.clearError()}
      aria-label="Fermer"
    >
      ✕
    </button>
  </div>
{/if}

<style>
  .error-notification {
    position: fixed;
    top: 1rem;
    left: 50%;
    transform: translateX(-50%);
    background-color: #fee2e2;
    border: 1px solid #ef4444;
    border-radius: 8px;
    padding: 0.75rem 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    max-width: 90%;
    min-width: 300px;
  }

  .error-content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .error-icon {
    font-size: 1.25rem;
  }

  .error-message {
    color: #991b1b;
    font-weight: 500;
    font-size: 0.875rem;
  }

  .close-button {
    background: none;
    border: none;
    color: #991b1b;
    cursor: pointer;
    font-size: 1rem;
    padding: 0.25rem;
    line-height: 1;
    opacity: 0.7;
    transition: opacity 0.2s;
  }

  .close-button:hover {
    opacity: 1;
  }

  @media (prefers-color-scheme: dark) {
    .error-notification {
      background-color: #450a0a;
      border-color: #dc2626;
    }

    .error-message {
      color: #fca5a5;
    }

    .close-button {
      color: #fca5a5;
    }
  }
</style>
