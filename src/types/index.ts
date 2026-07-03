/** Mirror of the Rust `CommandResult<T>` crossing the Tauri IPC boundary. */
export interface CommandResult<T> {
  success: boolean;
  data: T | null;
  stdout: string;
  stderr: string;
  exitCode: number | null;
  error: string | null;
}

/** A repository's local git identity. Either field may be unset. */
export interface GitIdentity {
  userName: string | null;
  email: string | null;
}

/** A snapshot of a repository's working tree and branch state. */
export interface GitStatus {
  branch: string | null;
  ahead: number;
  behind: number;
  isDirty: boolean;
  remoteUrl: string | null;
}

/** An account known to the GitHub CLI (`gh`). */
export interface GhAccount {
  username: string;
  host: string;
  active: boolean;
  protocol: string | null;
  scopes: string[];
}

/** One step of a sync operation. */
export interface SyncStep {
  name: string;
  success: boolean;
  stdout: string;
  stderr: string;
  exitCode: number | null;
}

/** The result of syncing a repository's identity. */
export interface SyncReport {
  steps: SyncStep[];
  overallSuccess: boolean;
}
