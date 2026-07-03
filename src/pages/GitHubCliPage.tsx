import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { Button, Card, CardBody, Chip, Spinner } from '@heroui/react';
import { Check, CircleUserRound, Link2, RefreshCw } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import { queryKeys } from '@/lib/queryKeys';
import { githubCliService } from '@/services/githubCli.service';
import type { GhAccount } from '@/types';

export function GitHubCliPage() {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  const accounts = useQuery({
    queryKey: queryKeys.ghAccounts,
    queryFn: githubCliService.authStatus,
  });

  const switchAccount = useMutation({
    mutationFn: githubCliService.authSwitch,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: queryKeys.ghAccounts }),
  });

  const setupGit = useMutation({ mutationFn: githubCliService.setupGit });

  const actions = (
    <>
      <Button
        variant="flat"
        startContent={<Link2 size={16} />}
        onPress={() => setupGit.mutate()}
        isLoading={setupGit.isPending}
      >
        {t('githubCli.setupGit')}
      </Button>
      <Button
        isIconOnly
        variant="light"
        aria-label={t('common.refresh')}
        onPress={() => accounts.refetch()}
      >
        <RefreshCw size={16} />
      </Button>
    </>
  );

  return (
    <Page title={t('nav.githubCli')} actions={actions}>
      {setupGit.isSuccess && <p className="text-sm text-success">{t('githubCli.setupGitDone')}</p>}
      {switchAccount.isError && (
        <p className="text-sm text-danger">{(switchAccount.error as Error).message}</p>
      )}

      {accounts.isPending ? (
        <Spinner />
      ) : accounts.isError ? (
        <p className="text-sm text-danger">{t('githubCli.unavailable')}</p>
      ) : accounts.data.length === 0 ? (
        <p className="text-sm text-default-500">{t('githubCli.emptyHint')}</p>
      ) : (
        <div className="flex flex-col gap-3">
          {accounts.data.map((account) => (
            <GhAccountRow
              key={`${account.host}/${account.username}`}
              account={account}
              isSwitching={switchAccount.isPending && switchAccount.variables === account.username}
              onSetActive={() => switchAccount.mutate(account.username)}
            />
          ))}
        </div>
      )}
    </Page>
  );
}

interface GhAccountRowProps {
  account: GhAccount;
  isSwitching: boolean;
  onSetActive: () => void;
}

function GhAccountRow({ account, isSwitching, onSetActive }: GhAccountRowProps) {
  const { t } = useTranslation();

  return (
    <Card shadow="sm">
      <CardBody className="flex flex-row items-center gap-3 p-4">
        <CircleUserRound size={28} className="shrink-0 text-default-500" />
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <p className="truncate font-medium">{account.username}</p>
            {account.active && (
              <Chip size="sm" color="success" variant="flat" startContent={<Check size={13} />}>
                {t('githubCli.active')}
              </Chip>
            )}
          </div>
          <p className="truncate text-xs text-default-500">
            {account.host}
            {account.protocol ? ` · ${account.protocol}` : ''}
          </p>
        </div>
        {!account.active && (
          <Button size="sm" variant="flat" onPress={onSetActive} isLoading={isSwitching}>
            {t('githubCli.setActive')}
          </Button>
        )}
      </CardBody>
    </Card>
  );
}
