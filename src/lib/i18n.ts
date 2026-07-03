import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

import en from '@/locales/en/translation.json';
import ja from '@/locales/ja/translation.json';

export const SUPPORTED_LANGUAGES = ['en', 'ja'] as const;
export type Language = (typeof SUPPORTED_LANGUAGES)[number];

void i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    ja: { translation: ja },
  },
  lng: 'ja',
  fallbackLng: 'en',
  interpolation: { escapeValue: false },
});

export default i18n;
