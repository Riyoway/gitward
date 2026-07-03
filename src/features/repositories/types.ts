/** A repository the user has registered with Gitward. */
export interface Repository {
  id: string;
  /** Display name, defaults to the folder name. */
  name: string;
  /** Absolute path on disk. */
  path: string;
  /** ISO timestamp of when it was added. */
  addedAt: string;
}
