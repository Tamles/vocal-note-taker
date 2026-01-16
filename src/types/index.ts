// Types TypeScript partagés

/**
 * Error types that match the Rust AppError enum variants.
 * Used for type-safe error handling between backend and frontend.
 */
export type AppErrorType =
  | 'MicrophoneAccessDenied'
  | 'MicrophoneNotFound'
  | 'TranscriptionFailed'
  | 'RecordingInterrupted'
  | 'ConfigurationError'
  | 'ClipboardError'
  | 'IoError';

/**
 * Application error structure received from backend via IPC.
 * All errors have actionable messages (FR47) to help users resolve issues.
 */
export interface AppError {
  type: AppErrorType;
  message: string;
  details?: string;
}

/**
 * Recording state for the application.
 */
export type RecordingState = 'idle' | 'recording' | 'transcribing';

// Placeholder for future type definitions:
// - AppConfig
// - WaveformData
// - TranscriptionResult
