import { Card, CardBody, Select, SelectItem, Switch } from '@heroui/react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import type { Language } from '@/lib/i18n';
import { firstSelectedKey } from '@/lib/selection';
import type { Theme } from '@/lib/theme';
import { useSettingsStore } from '@/stores/settingsStore';

const THEMES: Theme[] = ['system', 'light', 'dark'];
const LANGUAGES: { key: Language; label: string }[] = [
  { key: 'ja', label: '日本語' },
  { key: 'en', label: 'English' },
];

export function SettingsPage() {
  const { t } = useTranslation();
  const { theme, language, autoSwitch, setTheme, setLanguage, setAutoSwitch } = useSettingsStore();

  const themeLabels: Record<Theme, string> = {
    system: t('settings.themeSystem'),
    light: t('settings.themeLight'),
    dark: t('settings.themeDark'),
  };

  return (
    <Page title={t('nav.settings')}>
      <Card shadow="sm">
        <CardBody className="gap-5 p-6">
          <h2 className="text-sm font-medium text-default-500">{t('settings.appearance')}</h2>

          <Select
            label={t('settings.theme')}
            selectedKeys={[theme]}
            onSelectionChange={(keys) => {
              const value = firstSelectedKey<Theme>(keys);
              if (value) setTheme(value);
            }}
            className="max-w-xs"
          >
            {THEMES.map((value) => (
              <SelectItem key={value}>{themeLabels[value]}</SelectItem>
            ))}
          </Select>

          <Select
            label={t('settings.language')}
            selectedKeys={[language]}
            onSelectionChange={(keys) => {
              const value = firstSelectedKey<Language>(keys);
              if (value) setLanguage(value);
            }}
            className="max-w-xs"
          >
            {LANGUAGES.map(({ key, label }) => (
              <SelectItem key={key}>{label}</SelectItem>
            ))}
          </Select>
        </CardBody>
      </Card>

      <Card shadow="sm">
        <CardBody className="p-6">
          <h2 className="mb-4 text-sm font-medium text-default-500">{t('settings.behavior')}</h2>
          <Switch isSelected={autoSwitch} onValueChange={setAutoSwitch}>
            <div className="flex flex-col">
              <span className="text-sm">{t('settings.autoSwitch')}</span>
              <span className="text-xs text-default-400">{t('settings.autoSwitchDesc')}</span>
            </div>
          </Switch>
        </CardBody>
      </Card>
    </Page>
  );
}
