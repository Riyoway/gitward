import { create } from 'zustand';

import type { RepoSort } from '@/features/repositories/filtering';

/** Ephemeral view state for the Projects list (not persisted). */
interface UiState {
  search: string;
  sortBy: RepoSort;
  filterGitAccountId: string | null;
  favoritesOnly: boolean;

  setSearch: (search: string) => void;
  setSortBy: (sortBy: RepoSort) => void;
  setFilterGitAccountId: (id: string | null) => void;
  setFavoritesOnly: (value: boolean) => void;
}

export const useUiStore = create<UiState>((set) => ({
  search: '',
  sortBy: 'name',
  filterGitAccountId: null,
  favoritesOnly: false,

  setSearch: (search) => set({ search }),
  setSortBy: (sortBy) => set({ sortBy }),
  setFilterGitAccountId: (filterGitAccountId) => set({ filterGitAccountId }),
  setFavoritesOnly: (favoritesOnly) => set({ favoritesOnly }),
}));
