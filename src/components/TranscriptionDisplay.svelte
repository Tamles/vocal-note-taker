<script lang="ts">
  /**
   * TranscriptionDisplay component - Renders transcribed text
   *
   * @consumes transcriptionText - Subscribes to transcription result store
   * @accessibility aria-live="polite" for screen reader announcements
   */
  import { transcriptionText } from '../stores/transcriptionState';
</script>

{#if $transcriptionText}
  <div
    class="transcription-display"
    aria-live="polite"
    role="region"
    aria-label="Résultat de transcription"
  >
    <p class="transcription-text">{$transcriptionText}</p>
  </div>
{/if}

<style>
  .transcription-display {
    box-sizing: border-box;
    width: 90%;
    max-width: 600px;
    padding: 1.5rem;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    /* FR16: Pas de troncature - laisser le texte se déployer */
  }

  .transcription-text {
    color: var(--color-text);
    font-size: 1.1rem;
    line-height: 1.6;
    margin: 0;
    word-wrap: break-word;
    /* FR17: Lisibilité optimale pour scan rapide */
  }
</style>
