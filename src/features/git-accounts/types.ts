/** A named git identity the user can apply to repositories. */
export interface GitAccount {
  id: string;
  /** Display name, e.g. "Work" or "Personal". */
  label: string;
  /** git `user.name`. */
  userName: string;
  /** git `user.email`. */
  email: string;
  /** Accent color (hex) for quick visual identification. */
  color: string;
}

/** The editable fields of a {@link GitAccount}. */
export type GitAccountDraft = Omit<GitAccount, 'id'>;
