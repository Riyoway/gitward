import { open, save } from '@tauri-apps/plugin-dialog';

/** Prompt the user to pick a single directory. Returns null if cancelled. */
export async function pickDirectory(title?: string): Promise<string | null> {
  const selected = await open({ directory: true, multiple: false, title });
  return typeof selected === 'string' ? selected : null;
}

/** Prompt the user to pick a JSON file to open. Returns null if cancelled. */
export async function pickJsonFile(title?: string): Promise<string | null> {
  const selected = await open({
    multiple: false,
    title,
    filters: [{ name: 'JSON', extensions: ['json'] }],
  });
  return typeof selected === 'string' ? selected : null;
}

/** Prompt the user for a save path. Returns null if cancelled. */
export function pickSavePath(defaultName: string): Promise<string | null> {
  return save({ defaultPath: defaultName, filters: [{ name: 'JSON', extensions: ['json'] }] });
}
