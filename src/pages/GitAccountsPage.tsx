import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';

export function GitAccountsPage() {
  const { t } = useTranslation();
  return (
    <Page title={t('nav.gitAccounts')}>
      <p className="text-sm text-default-500">{t('common.empty')}</p>
    </Page>
  );
}
