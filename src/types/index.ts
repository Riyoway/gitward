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
