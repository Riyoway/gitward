import { invoke } from '@tauri-apps/api/core';

import type { GitIdentity, GitStatus } from '@/types';
import { invokeResult } from './tauri';

/** Typed wrappers over the `git_*` Tauri commands. */
export const gitService = {
  isRepo: (path: string) => invoke<boolean>('git_is_repo', { path }),

  readConfig: (path: string) => invokeResult<GitIdentity>('git_read_config', { path }),

  setConfig: (path: string, userName: string, email: string) =>
    invokeResult<void>('git_set_config', { path, userName, email }),

  status: (path: string) => invokeResult<GitStatus>('git_status', { path }),
};
