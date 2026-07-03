import type { GitAccountDraft } from './types';

export type FieldError = 'required' | 'invalidEmail';
export type GitAccountErrors = Partial<Record<'label' | 'userName' | 'email', FieldError>>;

// Deliberately permissive: one @, a dot in the domain, no spaces. git itself
// does not validate emails, so we only catch obvious mistakes.
const EMAIL = /^[^@\s]+@[^@\s]+\.[^@\s]+$/;

/** Validate a draft account, returning an error code per invalid field. */
export function validateGitAccount(draft: GitAccountDraft): GitAccountErrors {
  const errors: GitAccountErrors = {};
  if (!draft.label.trim()) errors.label = 'required';
  if (!draft.userName.trim()) errors.userName = 'required';
  if (!draft.email.trim()) errors.email = 'required';
  else if (!EMAIL.test(draft.email.trim())) errors.email = 'invalidEmail';
  return errors;
}
