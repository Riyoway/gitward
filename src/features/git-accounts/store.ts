import { create } from 'zustand';

import { appStore, StoreKey } from '@/services/store.service';
import type { GitAccount, GitAccountDraft } from './types';

interface GitAccountsState {
  accounts: GitAccount[];
  hydrated: boolean;

  hydrate: () => Promise<void>;
  add: (draft: GitAccountDraft) => void;
  update: (id: string, draft: GitAccountDraft) => void;
  remove: (id: string) => void;
}

function persist(accounts: GitAccount[]) {
  void appStore.set(StoreKey.GitAccounts, accounts);
}

export const useGitAccountsStore = create<GitAccountsState>((set, get) => ({
  accounts: [],
  hydrated: false,

  hydrate: async () => {
    try {
      const accounts = await appStore.get<GitAccount[]>(StoreKey.GitAccounts, []);
      set({ accounts, hydrated: true });
    } catch {
      set({ hydrated: true });
    }
  },

  add: (draft) => {
    const accounts = [...get().accounts, { id: crypto.randomUUID(), ...draft }];
    set({ accounts });
    persist(accounts);
  },

  update: (id, draft) => {
    const accounts = get().accounts.map((a) => (a.id === id ? { ...a, ...draft } : a));
    set({ accounts });
    persist(accounts);
  },

  remove: (id) => {
    const accounts = get().accounts.filter((a) => a.id !== id);
    set({ accounts });
    persist(accounts);
  },
}));
