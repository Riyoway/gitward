import { invoke } from '@tauri-apps/api/core';

import type { CommandResult } from '@/types';

/** Thrown when a command returns `success: false`. Carries the full result so
 *  callers (and the Logs view) can inspect stdout/stderr/exitCode. */
export class CommandError extends Error {
  constructor(readonly result: CommandResult<unknown>) {
    super(result.error ?? 'Command failed');
    this.name = 'CommandError';
  }
}

/**
 * Invoke a command that returns `CommandResult<T>`, returning its data on
 * success and throwing {@link CommandError} on failure. Use this for the common
 * case; call `invoke` directly for commands that return a bare value.
 */
export async function invokeResult<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const result = await invoke<CommandResult<T>>(command, args);
  if (!result.success) throw new CommandError(result);
  return result.data as T;
}
