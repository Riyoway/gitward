import { create } from 'zustand';

import i18n, { type Language } from '@/lib/i18n';
import { applyTheme, watchSystemTheme, type Theme } from '@/lib/theme';
import { appStore, StoreKey } from '@/services/store.service';

interface SettingsState {
  theme: Theme;
  language: Language;
  autoSwitch: boolean;
  /** Preferred terminal tool id for opening AI CLIs; '' = OS default. */
  terminalId: string;
  hydrated: boolean;

  /** Load persisted settings and apply side effects (theme, language). */
  hydrate: () => Promise<void>;
  setTheme: (theme: Theme) => void;
  setLanguage: (language: Language) => void;
  setAutoSwitch: (autoSwitch: boolean) => void;
  setTerminalId: (terminalId: string) => void;
}

const DEFAULTS = {
  theme: 'system' as Theme,
  language: 'ja' as Language,
  autoSwitch: true,
  terminalId: '',
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  ...DEFAULTS,
  hydrated: false,

  hydrate: async () => {
    try {
      const [theme, language, autoSwitch, terminalId] = await Promise.all([
        appStore.get<Theme>(StoreKey.Theme, DEFAULTS.theme),
        appStore.get<Language>(StoreKey.Language, DEFAULTS.language),
        appStore.get<boolean>(StoreKey.AutoSwitch, DEFAULTS.autoSwitch),
        appStore.get<string>(StoreKey.TerminalId, DEFAULTS.terminalId),
      ]);
      set({ theme, language, autoSwitch, terminalId, hydrated: true });
      applyTheme(theme);
      await i18n.changeLanguage(language);
    } catch {
      // Not running under Tauri (e.g. plain `vite` dev): keep the defaults.
      set({ hydrated: true });
      applyTheme(get().theme);
    }
  },

  setTheme: (theme) => {
    set({ theme });
    applyTheme(theme);
    void appStore.set(StoreKey.Theme, theme);
  },

  setLanguage: (language) => {
    set({ language });
    void i18n.changeLanguage(language);
    void appStore.set(StoreKey.Language, language);
  },

  setAutoSwitch: (autoSwitch) => {
    set({ autoSwitch });
    void appStore.set(StoreKey.AutoSwitch, autoSwitch);
  },

  setTerminalId: (terminalId) => {
    set({ terminalId });
    void appStore.set(StoreKey.TerminalId, terminalId);
  },
}));

// Follow the OS preference for as long as the theme is "system".
watchSystemTheme(() => useSettingsStore.getState().theme);
