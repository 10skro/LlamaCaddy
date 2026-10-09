import { invoke } from '@tauri-apps/api/core';
import type { AppSettings } from '@/types';

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings') as Promise<AppSettings>;
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  return invoke<void>('save_settings', { settings }) as Promise<void>;
}

/**
 * Opens a system folder selection dialog.
 *
 * @returns A promise resolving to the selected folder path, or `null` if the user cancelled.
 * @throws Will reject if the dialog fails to open (e.g., backend error).
 */
export function selectFolder(): Promise<string | null> {
  return invoke<string | null>('open_folder_dialog') as Promise<string | null>;
}

/**
 * Resolve the effective storage directory (configured path, or the app data
 * directory as fallback).
 */
export async function getStoragePath(): Promise<string> {
  return invoke<string>('get_storage_path') as Promise<string>;
}

/**
 * Open the storage folder in the system file manager.
 */
export async function openStorageFolder(): Promise<void> {
  return invoke<void>('open_storage_folder') as Promise<void>;
}

/**
 * Persist an explicit user theme choice to SQLite.
 * Called only when the user actively switches themes (not on startup).
 */
export async function persistThemeChange(themeId: string): Promise<void> {
  return invoke<void>('persist_theme_change', { themeId }) as Promise<void>;
}
