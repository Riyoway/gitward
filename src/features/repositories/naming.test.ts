import { describe, it, expect } from 'vitest';

import { deriveRepoName } from './naming';

describe('deriveRepoName', () => {
  it('takes the last segment of a Windows path', () => {
    expect(deriveRepoName('C:\\Users\\ada\\projects\\gitward')).toBe('gitward');
  });

  it('takes the last segment of a POSIX path', () => {
    expect(deriveRepoName('/home/ada/projects/gitward')).toBe('gitward');
  });

  it('ignores a trailing separator', () => {
    expect(deriveRepoName('/home/ada/gitward/')).toBe('gitward');
  });
});
