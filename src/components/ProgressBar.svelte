<script lang="ts">
  /**
   * ProgressBar component - Displays transcription progress.
   *
   * @prop progress - Progression actuelle (0-100)
   */

  /** Progression actuelle (0-100) */
  export let progress: number = 0;

  // Clamp entre 0 et 100
  $: clampedProgress = Math.min(100, Math.max(0, progress));
</script>

<div class="progress-container" role="progressbar" aria-valuenow={clampedProgress} aria-valuemin="0" aria-valuemax="100">
  <div class="progress-bar" style="width: {clampedProgress}%"></div>
  <span class="progress-text">{Math.round(clampedProgress)}%</span>
</div>

<style>
  .progress-container {
    width: 100%;
    max-width: 300px;
    height: 24px;
    background-color: #2d2d2d;
    border-radius: 12px;
    overflow: hidden;
    position: relative;
    border: 1px solid #444;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #4CAF50, #8BC34A);
    transition: width 0.3s ease-out, border-radius 0.3s ease-out;
    border-radius: 12px;
  }

  .progress-text {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    font-weight: bold;
    color: #fff;
    font-size: 14px;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  }
</style>
