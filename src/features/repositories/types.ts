/** A repository the user has registered with Gitward. */
export interface Repository {
  id: string;
  /** Display name, defaults to the folder name. */
  name: string;
  /** Absolute path on disk. */
  path: string;
  /** ISO timestamp of when it was added. */
  addedAt: string;
  /** Assigned git identity ([`GitAccount`](../git-accounts/types).id). */
  gitAccountId?: string;
  /** Assigned GitHub CLI account (username). */
  ghUsername?: string;
  /** Pinned to the top of the list. */
  favorite?: boolean;
  /** ISO timestamp of the last time a tool was launched here. */
  lastOpenedAt?: string;
}
