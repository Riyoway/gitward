import { describe, it, expect } from 'vitest';

import { filterAndSortRepos, type RepoFilter } from './filtering';
import type { Repository } from './types';

const base = (over: Partial<Repository>): Repository => ({
  id: over.id ?? 'x',
  name: over.name ?? 'repo',
  path: over.path ?? '/x',
  addedAt: '2026-01-01T00:00:00Z',
  ...over,
});

const repos: Repository[] = [
  base({ id: '1', name: 'alpha', path: '/work/alpha', gitAccountId: 'work' }),
  base({ id: '2', name: 'beta', path: '/home/beta', gitAccountId: 'personal', favorite: true }),
  base({ id: '3', name: 'gamma', path: '/work/gamma', gitAccountId: 'work', lastOpenedAt: '2026-06-01T00:00:00Z' }),
];

const defaults: RepoFilter = { search: '', sortBy: 'name', gitAccountId: null, favoritesOnly: false };

describe('filterAndSortRepos', () => {
  it('pins favorites first, then sorts by name', () => {
    const out = filterAndSortRepos(repos, defaults);
    expect(out.map((r) => r.name)).toEqual(['beta', 'alpha', 'gamma']);
  });

  it('searches name and path', () => {
    expect(filterAndSortRepos(repos, { ...defaults, search: 'work' }).map((r) => r.id)).toEqual([
      '1',
      '3',
    ]);
  });

  it('filters by git account', () => {
    expect(
      filterAndSortRepos(repos, { ...defaults, gitAccountId: 'personal' }).map((r) => r.id),
    ).toEqual(['2']);
  });

  it('shows only favorites when requested', () => {
    expect(filterAndSortRepos(repos, { ...defaults, favoritesOnly: true }).map((r) => r.id)).toEqual(
      ['2'],
    );
  });

  it('sorts by most-recently-opened (favorites still first)', () => {
    const out = filterAndSortRepos(repos, { ...defaults, sortBy: 'recent' });
    expect(out[0].name).toBe('beta'); // favorite pinned
    expect(out[1].name).toBe('gamma'); // has lastOpenedAt
  });
});
