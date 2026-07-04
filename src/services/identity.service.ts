import { invokeResult } from './tauri';
import type { IdentityField } from '@/types';

/** Detect a project's frameworks and read/write its app-identity fields. */
export const identityService = {
  detect: (path: string) => invokeResult<string[]>('detect_framework', { path }),
  read: (path: string) => invokeResult<IdentityField[]>('read_identity', { path }),
  write: (path: string, fields: { id: string; value: string }[]) =>
    invokeResult<void>('write_identity', { path, fields }),
};
