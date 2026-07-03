export type Theme = 'light' | 'dark' | 'system';

/** Resolve a (possibly `system`) theme to the concrete class to apply. */
export function resolveTheme(theme: Theme, prefersDark: boolean): 'light' | 'dark' {
  if (theme === 'system') return prefersDark ? 'dark' : 'light';
  return theme;
}

function prefersDark(): boolean {
  return window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? true;
}

/** Apply a theme by toggling the `dark`/`light` class HeroUI reads on <html>. */
export function applyTheme(theme: Theme): void {
  const resolved = resolveTheme(theme, prefersDark());
  const root = document.documentElement;
  root.classList.remove('light', 'dark');
  root.classList.add(resolved);
}

/**
 * Re-apply the theme when the OS preference changes, but only while the user
 * has chosen `system`. Returns an unsubscribe function.
 */
export function watchSystemTheme(getTheme: () => Theme): () => void {
  const media = window.matchMedia('(prefers-color-scheme: dark)');
  const listener = () => {
    if (getTheme() === 'system') applyTheme('system');
  };
  media.addEventListener('change', listener);
  return () => media.removeEventListener('change', listener);
}
