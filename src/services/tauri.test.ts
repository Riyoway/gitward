import { describe, it, expect, vi, beforeEach } from 'vitest';

import type { CommandResult } from '@/types';

const mockInvoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import { invokeResult, CommandError } from './tauri';

function result<T>(over: Partial<CommandResult<T>>): CommandResult<T> {
  return { success: true, data: null, stdout: '', stderr: '', exitCode: null, error: null, ...over };
}

describe('invokeResult', () => {
  beforeEach(() => mockInvoke.mockReset());

  it('returns data and forwards args on success', async () => {
    mockInvoke.mockResolvedValue(result({ data: { userName: 'Ada' } }));
    await expect(invokeResult('git_read_config', { path: '/x' })).resolves.toEqual({
      userName: 'Ada',
    });
    expect(mockInvoke).toHaveBeenCalledWith('git_read_config', { path: '/x' });
  });

  it('throws CommandError carrying the result on failure', async () => {
    mockInvoke.mockResolvedValue(
      result({ success: false, stderr: 'boom', exitCode: 128, error: 'failed' }),
    );
    await expect(invokeResult('git_status')).rejects.toBeInstanceOf(CommandError);
    await expect(invokeResult('git_status')).rejects.toMatchObject({
      result: { exitCode: 128, stderr: 'boom' },
    });
  });
});
