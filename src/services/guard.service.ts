import { invokeResult } from './tauri';

/** Whether a repo has our pre-push hook, a foreign one, or none. */
export type GuardState = 'none' | 'gitward' | 'foreign';

/** Push Guard: install/remove the pre-push hook that blocks identity-mismatched pushes. */
export const guardService = {
  status: (path: string) => invokeResult<GuardState>('guard_status', { path }),

  /** Install (or refresh) the hook with the repo's expected identity. */
  install: (path: string, userName: string, email: string, ghUsername?: string) =>
    invokeResult<void>('guard_install', {
      path,
      userName,
      email,
      ghUsername: ghUsername ?? null,
    }),

  uninstall: (path: string) => invokeResult<void>('guard_uninstall', { path }),
};
