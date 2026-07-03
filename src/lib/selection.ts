import type { Key } from 'react';

/** Read the first key from a HeroUI selection (single-select). */
export function firstSelectedKey<T extends string>(keys: 'all' | Set<Key>): T | undefined {
  if (keys === 'all') return undefined;
  return [...keys][0] as T | undefined;
}
