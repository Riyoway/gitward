import { create } from 'zustand';

import { appStore, StoreKey } from '@/services/store.service';
import type { Repository } from './types';

interface RepositoriesState {
  repositories: Repository[];
  hydrated: boolean;

  hydrate: () => Promise<void>;
  setAll: (repositories: Repository[]) => void;
  add: (repo: Repository) => void;
  remove: (id: string) => void;
  update: (id: string, patch: Partial<Omit<Repository, 'id'>>) => void;
  toggleFavorite: (id: string) => void;
  markOpened: (id: string) => void;
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

  setAll: (repositories) => {
    set({ repositories });
    persist(repositories);
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

  update: (id, patch) => {
    const repositories = get().repositories.map((r) => (r.id === id ? { ...r, ...patch } : r));
    set({ repositories });
    persist(repositories);
  },

  toggleFavorite: (id) => {
    const repositories = get().repositories.map((r) =>
      r.id === id ? { ...r, favorite: !r.favorite } : r,
    );
    set({ repositories });
    persist(repositories);
  },

  markOpened: (id) => {
    const now = new Date().toISOString();
    const repositories = get().repositories.map((r) =>
      r.id === id ? { ...r, lastOpenedAt: now } : r,
    );
    set({ repositories });
    persist(repositories);
  },
}));
