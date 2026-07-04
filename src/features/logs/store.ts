import { create } from 'zustand';

import { appStore, StoreKey } from '@/services/store.service';

export type LogAction = 'sync' | 'ghSwitch' | 'setupGit';

export interface LogEntry {
  id: string;
  /** ISO timestamp. */
  timestamp: string;
  action: LogAction;
  /** What the action operated on (repo name, gh username, …). */
  target: string;
  success: boolean;
  /** Optional failure detail or extra context. */
  detail?: string;
}

/** Keep the log bounded so the store file never grows without limit. */
const MAX_ENTRIES = 200;

interface LogsState {
  entries: LogEntry[];
  hydrated: boolean;

  hydrate: () => Promise<void>;
  record: (entry: Omit<LogEntry, 'id' | 'timestamp'>) => void;
  clear: () => void;
}

export const useLogsStore = create<LogsState>((set, get) => ({
  entries: [],
  hydrated: false,

  hydrate: async () => {
    try {
      const entries = await appStore.get<LogEntry[]>(StoreKey.Logs, []);
      set({ entries, hydrated: true });
    } catch {
      set({ hydrated: true });
    }
  },

  record: (entry) => {
    const full: LogEntry = {
      ...entry,
      id: crypto.randomUUID(),
      timestamp: new Date().toISOString(),
    };
    const entries = [full, ...get().entries].slice(0, MAX_ENTRIES);
    set({ entries });
    void appStore.set(StoreKey.Logs, entries);
  },

  clear: () => {
    set({ entries: [] });
    void appStore.set(StoreKey.Logs, []);
  },
}));
