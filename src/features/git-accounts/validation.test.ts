import { describe, it, expect } from 'vitest';

import { validateGitAccount } from './validation';
import type { GitAccountDraft } from './types';

const valid: GitAccountDraft = {
  label: 'Work',
  userName: 'Ada Lovelace',
  email: 'ada@example.com',
  color: '#6366f1',
};

describe('validateGitAccount', () => {
  it('accepts a complete, well-formed account', () => {
    expect(validateGitAccount(valid)).toEqual({});
  });

  it('flags missing required fields', () => {
    expect(validateGitAccount({ ...valid, label: '  ', userName: '' })).toEqual({
      label: 'required',
      userName: 'required',
    });
  });

  it('flags a malformed email', () => {
    expect(validateGitAccount({ ...valid, email: 'not-an-email' })).toEqual({
      email: 'invalidEmail',
    });
  });
});
