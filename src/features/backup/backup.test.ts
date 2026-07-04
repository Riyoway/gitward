import { describe, it, expect } from 'vitest';

import { parseBackup } from './backup';

describe('parseBackup', () => {
  it('accepts a well-formed backup', () => {
    const text = JSON.stringify({
      version: 1,
      exportedAt: '2026-01-01T00:00:00Z',
      gitAccounts: [],
      repositories: [],
      settings: { theme: 'system', language: 'ja', autoSwitch: true },
    });
    expect(parseBackup(text).version).toBe(1);
  });

  it('rejects invalid JSON', () => {
    expect(() => parseBackup('{not json')).toThrow(/valid JSON/);
  });

  it('rejects JSON that is not a backup', () => {
    expect(() => parseBackup('{"foo":1}')).toThrow(/Gitward backup/);
  });
});
