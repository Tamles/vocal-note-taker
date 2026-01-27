/**
 * Recording state store for centralized state management.
 * Manages recording/transcription state transitions and duration tracking.
 *
 * @listens recording-started - Transitions to 'recording'
 * @listens recording-stopped - Transitions to 'transcribing'
 * @listens transcription-complete - Transitions to 'idle'
 */
import { writable, derived } from 'svelte/store';
import type { RecordingState } from '../types';

const { subscribe, set } = writable<RecordingState>('idle');

/**
 * Recording state store with helper methods for state transitions.
 * State transitions: idle -> recording -> transcribing -> idle
 */
export const recordingState = {
  subscribe,
  setRecording: () => set('recording'),
  setTranscribing: () => set('transcribing'),
  setIdle: () => set('idle'),
};

/**
 * Derived store indicating if currently recording.
 * Use for conditional UI rendering.
 */
export const isRecording = derived(recordingState, ($state) => $state === 'recording');

/**
 * Derived store indicating if currently transcribing.
 * Use for conditional UI rendering.
 */
export const isTranscribing = derived(recordingState, ($state) => $state === 'transcribing');

/**
 * Recording duration store (in seconds).
 * Managed by Timer component via setInterval.
 */
const durationStore = writable<number>(0);

export const recordingDuration = {
  subscribe: durationStore.subscribe,
  increment: () => durationStore.update(n => n + 1),
  reset: () => durationStore.set(0),
};

/**
 * Audio waveform data store.
 * Receives samples from backend via waveform-data events.
 * Data is downsampled (~160 samples/sec at 16kHz source).
 * Uses sliding window of 200 samples for visualization.
 */
const WAVEFORM_WINDOW_SIZE = 200;
const audioDataStore = writable<number[]>([]);

export const audioData = {
  subscribe: audioDataStore.subscribe,
  set: (samples: number[]) => audioDataStore.set(samples),
  append: (samples: number[]) => audioDataStore.update(current => {
    // Optimisation: éviter allocations inutiles si possible
    const totalLength = current.length + samples.length;

    if (totalLength <= WAVEFORM_WINDOW_SIZE) {
      // Pas besoin de slice, juste concaténer
      current.push(...samples);
      return current;
    }

    // Sliding window: garder les derniers WAVEFORM_WINDOW_SIZE samples
    const overflow = totalLength - WAVEFORM_WINDOW_SIZE;
    const trimmedCurrent = overflow >= current.length
      ? []
      : current.slice(overflow);
    trimmedCurrent.push(...samples);
    return trimmedCurrent;
  }),
  clear: () => audioDataStore.set([]),
};
