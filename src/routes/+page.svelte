<script lang="ts">
  /**
   * Vocal Note Taker - Main Page
   *
   * Integrates error handling system with ErrorNotification component.
   * Errors from backend are pre-formatted as {type, message} - no parsing needed.
   */
  import { invoke } from '@tauri-apps/api/core';
  import ErrorNotification from '../components/ErrorNotification.svelte';
  import { errorStore } from '../stores/errorStore';
  import type { AppError } from '../types';

  /**
   * Validates and extracts AppError from Tauri error response.
   * Backend serializes errors as {type: "VariantName", message: "..."}.
   */
  function toAppError(err: unknown): AppError {
    if (typeof err === 'object' && err !== null && 'type' in err && 'message' in err) {
      return err as AppError;
    }
    // Fallback for unexpected error formats
    return {
      type: 'IoError',
      message: typeof err === 'string' ? err : 'Erreur inconnue'
    };
  }

  // TODO: Remove test function after Story 1.2 validation - kept for manual testing
  async function testError(errorType: string): Promise<void> {
    try {
      await invoke('test_error', { errorType });
    } catch (err) {
      errorStore.setError(toAppError(err));
    }
  }
</script>

<ErrorNotification />

<main class="container">
  <h1>Vocal Note Taker</h1>
  <p>Application de transcription vocale locale</p>

  <!-- TODO: Remove test buttons after Story 1.2 validation - kept for manual testing -->
  <div class="test-buttons">
    <p class="test-label">Test erreurs (temporaire):</p>
    <button on:click={() => testError('microphone_denied')}>Test Microphone Denied</button>
    <button on:click={() => testError('transcription')}>Test Transcription Failed</button>
    <button on:click={() => testError('clipboard')}>Test Clipboard Error</button>
  </div>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
}

/* TODO: Remove test buttons styling after Story 1.2 validation */
.test-buttons {
  margin-top: 2rem;
  padding: 1rem;
  border: 1px dashed #ccc;
  border-radius: 8px;
  background-color: #f9f9f9;
}

.test-label {
  font-size: 0.75rem;
  color: #666;
  margin-bottom: 0.5rem;
}

.test-buttons button {
  margin: 0.25rem;
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  cursor: pointer;
  border: 1px solid #ddd;
  border-radius: 4px;
  background: white;
}

.test-buttons button:hover {
  background: #f0f0f0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .test-buttons {
    background-color: #3f3f3f;
    border-color: #555;
  }

  .test-label {
    color: #aaa;
  }

  .test-buttons button {
    background: #4f4f4f;
    border-color: #666;
    color: #f6f6f6;
  }

  .test-buttons button:hover {
    background: #5f5f5f;
  }
}
</style>
