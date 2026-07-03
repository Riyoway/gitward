import { useQuery, useQueryClient } from '@tanstack/react-query';
import { Button, Card, CardBody, Chip, Spinner, Tooltip } from '@heroui/react';
import { GitBranch, RefreshCw, Trash2, User } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { queryKeys } from '@/lib/queryKeys';
import { gitService } from '@/services/git.service';
import type { Repository } from '../types';

interface RepositoryCardProps {
  repo: Repository;
  onRemove: (id: string) => void;
}

export function RepositoryCard({ repo, onRemove }: RepositoryCardProps) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  const status = useQuery({
    queryKey: queryKeys.repoStatus(repo.id),
    queryFn: () => gitService.status(repo.path),
  });
  const identity = useQuery({
    queryKey: queryKeys.repoIdentity(repo.id),
    queryFn: () => gitService.readConfig(repo.path),
  });

  const refresh = () => {
    void queryClient.invalidateQueries({ queryKey: queryKeys.repoStatus(repo.id) });
    void queryClient.invalidateQueries({ queryKey: queryKeys.repoIdentity(repo.id) });
  };

  return (
    <Card shadow="sm" className="w-full">
      <CardBody className="gap-3 p-5">
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0">
            <h3 className="truncate font-medium">{repo.name}</h3>
            <p className="truncate font-mono text-xs text-default-400" title={repo.path}>
              {repo.path}
            </p>
          </div>
          <div className="flex shrink-0 items-center gap-1">
            <Tooltip content={t('common.refresh')} closeDelay={0}>
              <Button
                isIconOnly
                size="sm"
                variant="light"
                aria-label={t('common.refresh')}
                onPress={refresh}
              >
                <RefreshCw size={15} />
              </Button>
            </Tooltip>
            <Tooltip content={t('common.delete')} closeDelay={0}>
              <Button
                isIconOnly
                size="sm"
                variant="light"
                color="danger"
                aria-label={t('common.delete')}
                onPress={() => onRemove(repo.id)}
              >
                <Trash2 size={15} />
              </Button>
            </Tooltip>
          </div>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          {status.isPending ? (
            <Spinner size="sm" />
          ) : status.isError ? (
            <Chip size="sm" color="danger" variant="flat">
              {t('repository.statusUnavailable')}
            </Chip>
          ) : (
            <>
              <Chip size="sm" variant="flat" startContent={<GitBranch size={13} />}>
                {status.data.branch ?? t('repository.detached')}
              </Chip>
              {status.data.ahead > 0 && (
                <Chip size="sm" variant="flat" color="primary">
                  ↑{status.data.ahead}
                </Chip>
              )}
              {status.data.behind > 0 && (
                <Chip size="sm" variant="flat" color="warning">
                  ↓{status.data.behind}
                </Chip>
              )}
              <Chip size="sm" variant="dot" color={status.data.isDirty ? 'warning' : 'success'}>
                {status.data.isDirty ? t('repository.dirty') : t('repository.clean')}
              </Chip>
            </>
          )}
        </div>

        <div className="flex items-center gap-2 text-xs text-default-500">
          <User size={13} className="shrink-0" />
          {identity.data ? (
            identity.data.userName || identity.data.email ? (
              <span className="truncate">
                {identity.data.userName ?? '—'}
                {identity.data.email ? ` <${identity.data.email}>` : ''}
              </span>
            ) : (
              <span className="text-default-400">{t('repository.identityUnset')}</span>
            )
          ) : (
            <span className="text-default-400">{t('common.loading')}</span>
          )}
        </div>
      </CardBody>
    </Card>
  );
}
