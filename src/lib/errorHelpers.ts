/**
 * Utilitaires de gestion des erreurs.
 * Fonctions partagées pour la conversion et le traitement des erreurs.
 */
import type { AppError } from '../types';

/**
 * Convertit une erreur inconnue en format AppError.
 * Utilisé pour normaliser les erreurs reçues du backend Tauri.
 *
 * @param err - Erreur de type inconnu à convertir
 * @returns AppError formaté avec type et message
 */
export function toAppError(err: unknown): AppError {
  if (typeof err === 'object' && err !== null && 'type' in err && 'message' in err) {
    return err as AppError;
  }
  return {
    type: 'IoError',
    message: typeof err === 'string' ? err : 'Erreur inconnue'
  };
}
