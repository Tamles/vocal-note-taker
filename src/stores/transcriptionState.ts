/**
 * Transcription state store for centralized state management.
 * Manages transcription progress and results.
 *
 * @listens transcription-progress - Updates progress (0-100)
 * @listens transcription-complete - Stores transcribed text
 */
import { writable } from 'svelte/store';

/**
 * Transcription progress store (0-100).
 * Updated via transcription-progress events from backend.
 */
const progressStore = writable<number>(0);

export const transcriptionProgress = {
  subscribe: progressStore.subscribe,
  set: (value: number) => progressStore.set(value),
  reset: () => progressStore.set(0),
};

/**
 * Transcribed text store.
 * Updated via transcription-complete events from backend.
 */
const textStore = writable<string>('');

export const transcriptionText = {
  subscribe: textStore.subscribe,
  set: (value: string) => textStore.set(value),
  reset: () => textStore.set(''),
};

/**
 * Reset all transcription state.
 * Call before starting a new transcription.
 */
export function resetTranscription(): void {
  transcriptionProgress.reset();
  transcriptionText.reset();
}
