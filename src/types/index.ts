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

/** Cross-check of the active gh account against Git Credential Manager. */
export interface CredentialDiagnosis {
  ghUser: string | null;
  gcmUsers: string[];
  mismatch: boolean;
}

/** An editable app-identity field found in a project's config files. */
export interface IdentityField {
  id: string;
  label: string;
  file: string;
  value: string | null;
}

/** A developer tool Gitward can launch, and whether it is installed. */
export interface Tool {
  id: string;
  name: string;
  category: string;
  installed: boolean;
}

/** Environment health check. */
export interface HealthReport {
  git: boolean;
  gh: boolean;
  ssh: boolean;
  internet: boolean;
  tools: Tool[];
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
