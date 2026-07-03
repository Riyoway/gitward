import { open } from '@tauri-apps/plugin-dialog';

/** Prompt the user to pick a single directory. Returns null if cancelled. */
export async function pickDirectory(title?: string): Promise<string | null> {
  const selected = await open({ directory: true, multiple: false, title });
  return typeof selected === 'string' ? selected : null;
}
