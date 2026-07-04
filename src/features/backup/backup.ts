import type { Language } from '@/lib/i18n';
import type { Theme } from '@/lib/theme';
import { useGitAccountsStore } from '@/features/git-accounts/store';
import { useRepositoriesStore } from '@/features/repositories/store';
import { useSettingsStore } from '@/stores/settingsStore';
import type { GitAccount } from '@/features/git-accounts/types';
import type { Repository } from '@/features/repositories/types';

export const BACKUP_VERSION = 1;

export interface GitwardBackup {
  version: number;
  exportedAt: string;
  gitAccounts: GitAccount[];
  repositories: Repository[];
  settings: { theme: Theme; language: Language; autoSwitch: boolean };
}

/** Snapshot the current stores into a serializable backup. */
export function buildBackup(): GitwardBackup {
  const { theme, language, autoSwitch } = useSettingsStore.getState();
  return {
    version: BACKUP_VERSION,
    exportedAt: new Date().toISOString(),
    gitAccounts: useGitAccountsStore.getState().accounts,
    repositories: useRepositoriesStore.getState().repositories,
    settings: { theme, language, autoSwitch },
  };
}

/** Parse and validate backup JSON. Throws on malformed input. */
export function parseBackup(text: string): GitwardBackup {
  let data: unknown;
  try {
    data = JSON.parse(text);
  } catch {
    throw new Error('Not valid JSON');
  }
  if (
    typeof data !== 'object' ||
    data === null ||
    !Array.isArray((data as GitwardBackup).gitAccounts) ||
    !Array.isArray((data as GitwardBackup).repositories)
  ) {
    throw new Error('Not a Gitward backup');
  }
  return data as GitwardBackup;
}

/** Load a parsed backup into the stores (replaces existing data). */
export function applyBackup(backup: GitwardBackup): void {
  useGitAccountsStore.getState().setAll(backup.gitAccounts);
  useRepositoriesStore.getState().setAll(backup.repositories);
  const settings = useSettingsStore.getState();
  if (backup.settings?.theme) settings.setTheme(backup.settings.theme);
  if (backup.settings?.language) settings.setLanguage(backup.settings.language);
  if (typeof backup.settings?.autoSwitch === 'boolean') {
    settings.setAutoSwitch(backup.settings.autoSwitch);
  }
}
