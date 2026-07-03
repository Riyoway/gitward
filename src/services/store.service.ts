import { LazyStore } from '@tauri-apps/plugin-store';

/**
 * The single persistent store, backed by `settings.json`. `autoSave` debounces
 * writes to disk, so callers just `set` and forget. Loaded lazily on first use.
 */
const store = new LazyStore('settings.json', { autoSave: 500, defaults: {} });

/** Persisted top-level keys. Keeping them here avoids stringly-typed drift. */
export const StoreKey = {
  Theme: 'theme',
  Language: 'language',
  AutoSwitch: 'autoSwitch',
} as const;

export const appStore = {
  /** Read a key, falling back to `fallback` when unset. */
  async get<T>(key: string, fallback: T): Promise<T> {
    const value = await store.get<T>(key);
    return value ?? fallback;
  },

  set(key: string, value: unknown): Promise<void> {
    return store.set(key, value);
  },
};
