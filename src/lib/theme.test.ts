import { describe, it, expect } from 'vitest';

import { resolveTheme } from './theme';

describe('resolveTheme', () => {
  it('passes through explicit choices', () => {
    expect(resolveTheme('dark', false)).toBe('dark');
    expect(resolveTheme('light', true)).toBe('light');
  });

  it('follows the OS preference when set to system', () => {
    expect(resolveTheme('system', true)).toBe('dark');
    expect(resolveTheme('system', false)).toBe('light');
  });
});
