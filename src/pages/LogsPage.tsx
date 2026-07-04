import { Button, Card, CardBody, Chip } from '@heroui/react';
import { Trash2 } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import { useLogsStore } from '@/features/logs/store';

export function LogsPage() {
  const { t, i18n } = useTranslation();
  const entries = useLogsStore((s) => s.entries);
  const clear = useLogsStore((s) => s.clear);

  const formatTime = (iso: string) => new Date(iso).toLocaleString(i18n.language);

  return (
    <Page
      title={t('nav.logs')}
      actions={
        entries.length > 0 && (
          <Button
            variant="light"
            color="danger"
            startContent={<Trash2 size={16} />}
            onPress={clear}
          >
            {t('logs.clear')}
          </Button>
        )
      }
    >
      {entries.length === 0 ? (
        <p className="text-sm text-default-500">{t('logs.empty')}</p>
      ) : (
        <div className="flex flex-col gap-2">
          {entries.map((entry) => (
            <Card key={entry.id} shadow="sm">
              <CardBody className="flex flex-row items-start gap-3 p-3">
                <Chip size="sm" color={entry.success ? 'success' : 'danger'} variant="flat">
                  {entry.success ? t('logs.ok') : t('logs.failed')}
                </Chip>
                <div className="min-w-0 flex-1">
                  <p className="text-sm">
                    <span className="font-medium">{t(`logs.action.${entry.action}`)}</span>
                    <span className="text-default-500"> · {entry.target}</span>
                  </p>
                  {entry.detail && (
                    <p className="mt-1 whitespace-pre-wrap break-words font-mono text-xs text-default-500">
                      {entry.detail}
                    </p>
                  )}
                </div>
                <span className="shrink-0 text-xs text-default-400">
                  {formatTime(entry.timestamp)}
                </span>
              </CardBody>
            </Card>
          ))}
        </div>
      )}
    </Page>
  );
}
