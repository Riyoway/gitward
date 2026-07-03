import { invokeResult } from './tauri';
import type { GhAccount } from '@/types';

/** Typed wrappers over the `gh_*` Tauri commands. */
export const githubCliService = {
  authStatus: () => invokeResult<GhAccount[]>('gh_auth_status'),
  authSwitch: (username: string) => invokeResult<void>('gh_auth_switch', { username }),
  setupGit: () => invokeResult<void>('gh_auth_setup_git'),
};
