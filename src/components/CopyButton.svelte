<script lang="ts">
  /**
   * CopyButton component - Copy transcription to clipboard
   *
   * @consumes transcriptionText - Gets text to copy
   * @calls copy_to_clipboard - Invokes backend clipboard command
   * @displays "✓ Copié!" feedback on success (FR23)
   * @exports focus() - Method to programmatically focus the button (FR22)
   */
  import { onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { transcriptionText } from '../stores/transcriptionState';
  import { errorStore } from '../stores/errorStore';
  import { toAppError } from '../lib/errorHelpers';

  let copied = false;
  let copyTimeout: ReturnType<typeof setTimeout> | null = null;
  let buttonElement: HTMLButtonElement;

  /**
   * Expose focus method for parent component to trigger auto-focus (FR22).
   * Called after transcription completes to enable Enter key workflow.
   */
  export function focus() {
    buttonElement?.focus();
  }

  // Cleanup timeout on component destroy to prevent memory leak
  onDestroy(() => {
    if (copyTimeout) clearTimeout(copyTimeout);
  });

  async function handleCopy() {
    if (!$transcriptionText) return;

    try {
      await invoke('copy_to_clipboard', { text: $transcriptionText });
      copied = true;

      // Reset feedback après 2.5 secondes (FR23)
      if (copyTimeout) clearTimeout(copyTimeout);
      copyTimeout = setTimeout(() => {
        copied = false;
      }, 2500);
    } catch (error) {
      errorStore.setError(toAppError(error));
    }
  }
</script>

<button
  bind:this={buttonElement}
  class="copy-button"
  class:copied
  on:click={handleCopy}
  disabled={!$transcriptionText}
  aria-label={copied ? 'Copié dans le presse-papiers' : 'Copier le texte'}
>
  {copied ? '✓ Copié!' : 'Copier'}
</button>

<style>
  .copy-button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    font-weight: 500;
    color: var(--color-text);
    background: var(--color-accent);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 120px;
  }

  .copy-button:hover:not(:disabled) {
    background: var(--color-accent-hover, #1a4a7a);
    border-color: var(--color-accent-border, #4a90c2);
  }

  .copy-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .copy-button.copied {
    background: #22c55e;
    border-color: #16a34a;
    color: #fff;
  }

  /* Focus styles for keyboard navigation (FR22) */
  .copy-button:focus:not(:disabled) {
    outline: none;
    box-shadow: 0 0 0 3px var(--color-focus-glow, rgba(74, 144, 194, 0.5));
  }

  .copy-button:focus-visible:not(:disabled) {
    outline: 2px solid var(--color-focus, #4a90c2);
    outline-offset: 2px;
  }

  /* Green focus glow when in copied state */
  .copy-button.copied:focus:not(:disabled) {
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.5);
  }

  .copy-button.copied:focus-visible:not(:disabled) {
    outline-color: #22c55e;
  }
</style>
