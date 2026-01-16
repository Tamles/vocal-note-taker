/**
 * Error store for centralized error handling.
 * Manages error state with auto-clear functionality.
 *
 * @listens error - Receives error events from backend
 */
import { writable } from 'svelte/store';
import type { AppError } from '../types';

const AUTO_CLEAR_TIMEOUT = 5000; // 5 seconds

const { subscribe, set } = writable<AppError | null>(null);

let timeoutId: ReturnType<typeof setTimeout> | null = null;

/**
 * Clears any pending auto-clear timeout.
 */
function clearPendingTimeout(): void {
  if (timeoutId) {
    clearTimeout(timeoutId);
    timeoutId = null;
  }
}

/**
 * Error store with setError and clearError methods.
 * Auto-clears errors after 5 seconds by default.
 */
export const errorStore = {
  subscribe,

  /**
   * Sets an error and schedules auto-clear.
   * @param error - The error to display
   */
  setError: (error: AppError): void => {
    clearPendingTimeout();
    set(error);
    timeoutId = setTimeout(() => {
      set(null);
      timeoutId = null;
    }, AUTO_CLEAR_TIMEOUT);
  },

  /**
   * Manually clears the current error.
   */
  clearError: (): void => {
    clearPendingTimeout();
    set(null);
  }
};
