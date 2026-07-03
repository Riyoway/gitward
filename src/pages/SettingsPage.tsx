import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';

export function SettingsPage() {
  const { t } = useTranslation();
  return (
    <Page title={t('nav.settings')}>
      <p className="text-sm text-default-500">{t('common.empty')}</p>
    </Page>
  );
}
