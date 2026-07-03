import { invokeResult } from './tauri';
import type { SyncReport } from '@/types';

/** Apply a repository's assigned identity (git config, then gh switch + setup-git). */
export const syncService = {
  syncRepository: (path: string, userName: string, email: string, ghUsername?: string) =>
    invokeResult<SyncReport>('sync_repository', {
      path,
      userName,
      email,
      ghUsername: ghUsername ?? null,
    }),
};
