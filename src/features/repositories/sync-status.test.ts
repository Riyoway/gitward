import { describe, it, expect } from 'vitest';

import { identityMatches } from './sync-status';
import type { GitAccount } from '@/features/git-accounts/types';

const account: GitAccount = {
  id: 'a1',
  label: 'Work',
  userName: 'Ada Lovelace',
  email: 'ada@example.com',
  color: '#6366f1',
};

describe('identityMatches', () => {
  it('matches when name and email are identical', () => {
    expect(identityMatches({ userName: 'Ada Lovelace', email: 'ada@example.com' }, account)).toBe(
      true,
    );
  });

  it('does not match on a different email', () => {
    expect(identityMatches({ userName: 'Ada Lovelace', email: 'other@example.com' }, account)).toBe(
      false,
    );
  });

  it('does not match when the identity is unset', () => {
    expect(identityMatches({ userName: null, email: null }, account)).toBe(false);
  });
});
