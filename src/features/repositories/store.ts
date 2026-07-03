import { create } from 'zustand';

import { appStore, StoreKey } from '@/services/store.service';
import type { Repository } from './types';

interface RepositoriesState {
  repositories: Repository[];
  hydrated: boolean;

  hydrate: () => Promise<void>;
  add: (repo: Repository) => void;
  remove: (id: string) => void;
  rename: (id: string, name: string) => void;
}

function persist(repositories: Repository[]) {
  void appStore.set(StoreKey.Repositories, repositories);
}

export const useRepositoriesStore = create<RepositoriesState>((set, get) => ({
  repositories: [],
  hydrated: false,

  hydrate: async () => {
    try {
      const repositories = await appStore.get<Repository[]>(StoreKey.Repositories, []);
      set({ repositories, hydrated: true });
    } catch {
      set({ hydrated: true });
    }
  },

  add: (repo) => {
    const repositories = [...get().repositories, repo];
    set({ repositories });
    persist(repositories);
  },

  remove: (id) => {
    const repositories = get().repositories.filter((r) => r.id !== id);
    set({ repositories });
    persist(repositories);
  },

  rename: (id, name) => {
    const repositories = get().repositories.map((r) => (r.id === id ? { ...r, name } : r));
    set({ repositories });
    persist(repositories);
  },
}));
