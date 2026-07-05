import { invokeResult } from './tauri';

/** Read/write text files at OS-dialog-chosen paths (import/export). */
export const fsService = {
  readTextFile: (path: string) => invokeResult<string>('read_text_file', { path }),
  writeTextFile: (path: string, contents: string) =>
    invokeResult<void>('write_text_file', { path, contents }),
  readImageDataUrl: (path: string) => invokeResult<string>('read_image_data_url', { path }),
};
