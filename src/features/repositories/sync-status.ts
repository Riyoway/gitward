import type { GitAccount } from '@/features/git-accounts/types';
import type { GitIdentity } from '@/types';

/** True when a repo's current local identity already equals the assigned account. */
export function identityMatches(identity: GitIdentity, account: GitAccount): boolean {
  return identity.userName === account.userName && identity.email === account.email;
}
