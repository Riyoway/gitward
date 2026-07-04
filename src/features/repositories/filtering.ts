import type { Repository } from './types';

export type RepoSort = 'name' | 'recent';

export interface RepoFilter {
  search: string;
  sortBy: RepoSort;
  gitAccountId: string | null;
  favoritesOnly: boolean;
}

/** Filter and sort repositories. Favorites are always pinned to the top. */
export function filterAndSortRepos(repos: Repository[], f: RepoFilter): Repository[] {
  const q = f.search.trim().toLowerCase();

  const filtered = repos.filter((r) => {
    if (f.favoritesOnly && !r.favorite) return false;
    if (f.gitAccountId && r.gitAccountId !== f.gitAccountId) return false;
    if (q && !(r.name.toLowerCase().includes(q) || r.path.toLowerCase().includes(q))) return false;
    return true;
  });

  return [...filtered].sort((a, b) => {
    if (!!a.favorite !== !!b.favorite) return a.favorite ? -1 : 1;
    if (f.sortBy === 'recent') return (b.lastOpenedAt ?? '').localeCompare(a.lastOpenedAt ?? '');
    return a.name.localeCompare(b.name);
  });
}
