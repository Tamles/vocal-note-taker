/**
 * Recording state store for centralized state management.
 * Manages recording/transcription state transitions.
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
