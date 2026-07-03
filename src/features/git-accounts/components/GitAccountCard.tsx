import { Button, Card, CardBody, Tooltip } from '@heroui/react';
import { Pencil, Trash2 } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import type { GitAccount } from '../types';

interface GitAccountCardProps {
  account: GitAccount;
  onEdit: (account: GitAccount) => void;
  onRemove: (id: string) => void;
}

export function GitAccountCard({ account, onEdit, onRemove }: GitAccountCardProps) {
  const { t } = useTranslation();

  return (
    <Card shadow="sm">
      <CardBody className="flex flex-row items-center gap-3 p-4">
        <span
          className="h-8 w-8 shrink-0 rounded-full"
          style={{ backgroundColor: account.color }}
          aria-hidden
        />
        <div className="min-w-0 flex-1">
          <p className="truncate font-medium">{account.label}</p>
          <p className="truncate text-xs text-default-500">
            {account.userName} &lt;{account.email}&gt;
          </p>
        </div>
        <div className="flex shrink-0 items-center gap-1">
          <Tooltip content={t('common.edit')} closeDelay={0}>
            <Button
              isIconOnly
              size="sm"
              variant="light"
              aria-label={t('common.edit')}
              onPress={() => onEdit(account)}
            >
              <Pencil size={15} />
            </Button>
          </Tooltip>
          <Tooltip content={t('common.delete')} closeDelay={0}>
            <Button
              isIconOnly
              size="sm"
              variant="light"
              color="danger"
              aria-label={t('common.delete')}
              onPress={() => onRemove(account.id)}
            >
              <Trash2 size={15} />
            </Button>
          </Tooltip>
        </div>
      </CardBody>
    </Card>
  );
}
