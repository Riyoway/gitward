import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';

export function LogsPage() {
  const { t } = useTranslation();
  return (
    <Page title={t('nav.logs')}>
      <p className="text-sm text-default-500">{t('common.empty')}</p>
    </Page>
  );
}
