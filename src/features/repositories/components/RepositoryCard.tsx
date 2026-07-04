import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  Button,
  Card,
  CardBody,
  Chip,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
  Select,
  SelectItem,
  Spinner,
  Tooltip,
} from '@heroui/react';
import { ArrowRightLeft, ChevronDown, FolderOpen, GitBranch, RefreshCw, Star, Trash2, User } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { queryKeys } from '@/lib/queryKeys';
import { firstSelectedKey } from '@/lib/selection';
import { toastError, toastSuccess } from '@/lib/toast';
import { gitService } from '@/services/git.service';
import { githubCliService } from '@/services/githubCli.service';
import { launcherService } from '@/services/launcher.service';
import { syncService } from '@/services/sync.service';
import { useGitAccountsStore } from '@/features/git-accounts/store';
import { useLogsStore } from '@/features/logs/store';
import { useSettingsStore } from '@/stores/settingsStore';
import type { SyncReport } from '@/types';
import { useRepositoriesStore } from '../store';
import { identityMatches } from '../sync-status';
import type { Repository } from '../types';

/** First failing step's detail, for inline error display. */
function failureMessage(report: SyncReport): string {
  const failed = report.steps.find((s) => !s.success);
  if (!failed) return '';
  const detail = failed.stderr.trim() || failed.stdout.trim();
  return detail ? `${failed.name}: ${detail}` : failed.name;
}

interface RepositoryCardProps {
  repo: Repository;
  onRemove: (id: string) => void;
}

export function RepositoryCard({ repo, onRemove }: RepositoryCardProps) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const accounts = useGitAccountsStore((s) => s.accounts);
  const update = useRepositoriesStore((s) => s.update);
  const toggleFavorite = useRepositoriesStore((s) => s.toggleFavorite);
  const markOpened = useRepositoriesStore((s) => s.markOpened);
  const recordLog = useLogsStore((s) => s.record);
  const autoSwitch = useSettingsStore((s) => s.autoSwitch);

  const status = useQuery({
    queryKey: queryKeys.repoStatus(repo.id),
    queryFn: () => gitService.status(repo.path),
  });
  const identity = useQuery({
    queryKey: queryKeys.repoIdentity(repo.id),
    queryFn: () => gitService.readConfig(repo.path),
  });
  // Shared across all cards (same key): one request regardless of card count.
  const ghAccounts = useQuery({
    queryKey: queryKeys.ghAccounts,
    queryFn: githubCliService.authStatus,
  });
  const tools = useQuery({ queryKey: queryKeys.tools, queryFn: launcherService.detectTools });

  const [openError, setOpenError] = useState<string | null>(null);

  const refresh = () => {
    void queryClient.invalidateQueries({ queryKey: queryKeys.repoStatus(repo.id) });
    void queryClient.invalidateQueries({ queryKey: queryKeys.repoIdentity(repo.id) });
  };

  const assignedAccount = accounts.find((a) => a.id === repo.gitAccountId);

  const sync = useMutation({
    mutationFn: () => {
      if (!assignedAccount) throw new Error('No git account assigned');
      return syncService.syncRepository(
        repo.path,
        assignedAccount.userName,
        assignedAccount.email,
        repo.ghUsername,
      );
    },
    onSuccess: (report) => {
      void queryClient.invalidateQueries({ queryKey: queryKeys.repoIdentity(repo.id) });
      void queryClient.invalidateQueries({ queryKey: queryKeys.repoStatus(repo.id) });
      void queryClient.invalidateQueries({ queryKey: queryKeys.ghAccounts });
      recordLog({
        action: 'sync',
        target: repo.name,
        success: report.overallSuccess,
        detail: report.overallSuccess ? undefined : failureMessage(report),
      });
      if (report.overallSuccess) toastSuccess(t('repository.syncDone', { name: repo.name }));
      else toastError(t('repository.syncFailed'), failureMessage(report));
    },
    onError: (error) => {
      recordLog({ action: 'sync', target: repo.name, success: false, detail: (error as Error).message });
      toastError(t('repository.syncFailed'), (error as Error).message);
    },
  });
  const syncState =
    assignedAccount && identity.data
      ? identityMatches(identity.data, assignedAccount)
        ? 'synced'
        : 'needsSync'
      : null;

  const remoteUrl = status.data?.remoteUrl ?? null;
  const canOpenRemote = !!remoteUrl && /^https?:\/\//.test(remoteUrl);
  const openItems = [
    ...(tools.data ?? [])
      .filter((tool) => tool.installed)
      .map((tool) => ({ key: `tool:${tool.id}`, label: t('launcher.openIn', { name: tool.name }) })),
    { key: 'reveal', label: t('launcher.reveal') },
    ...(canOpenRemote ? [{ key: 'remote', label: t('launcher.openRemote') }] : []),
  ];

  const handleOpen = async (key: string) => {
    setOpenError(null);
    try {
      if (key.startsWith('tool:')) {
        // Auto-switch: align identity before opening if it has drifted.
        if (autoSwitch && assignedAccount && syncState === 'needsSync') {
          await sync.mutateAsync();
        }
        await launcherService.launchTool(key.slice(5), repo.path);
        markOpened(repo.id);
      } else if (key === 'reveal') await launcherService.revealInExplorer(repo.path);
      else if (key === 'remote' && remoteUrl) await launcherService.openRemote(remoteUrl);
    } catch (e) {
      setOpenError(e instanceof Error ? e.message : String(e));
    }
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
            <Tooltip content={t('repository.favorite')} closeDelay={0}>
              <Button
                isIconOnly
                size="sm"
                variant="light"
                aria-label={t('repository.favorite')}
                onPress={() => toggleFavorite(repo.id)}
              >
                <Star
                  size={15}
                  className={repo.favorite ? 'text-warning' : ''}
                  fill={repo.favorite ? 'currentColor' : 'none'}
                />
              </Button>
            </Tooltip>
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
              {syncState === 'needsSync' && (
                <Chip size="sm" color="warning" variant="flat">
                  {t('repository.needsSync')}
                </Chip>
              )}
              {syncState === 'synced' && (
                <Chip size="sm" color="success" variant="flat">
                  {t('repository.inSync')}
                </Chip>
              )}
            </>
          )}
        </div>

        <div className="flex items-center gap-2 text-xs text-default-500">
          <User size={13} className="shrink-0" />
          {identity.data ? (
            identity.data.userName || identity.data.email ? (
              <span className="truncate">
                {identity.data.userName ?? '-'}
                {identity.data.email ? ` <${identity.data.email}>` : ''}
              </span>
            ) : (
              <span className="text-default-400">{t('repository.identityUnset')}</span>
            )
          ) : (
            <span className="text-default-400">{t('common.loading')}</span>
          )}
        </div>

        <div className="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <Select
            size="sm"
            label={t('repository.gitAccount')}
            aria-label={t('repository.gitAccount')}
            placeholder={t('repository.unassigned')}
            selectedKeys={repo.gitAccountId ? [repo.gitAccountId] : []}
            isDisabled={accounts.length === 0}
            onSelectionChange={(keys) => {
              const id = firstSelectedKey(keys);
              if (id) update(repo.id, { gitAccountId: id });
            }}
          >
            {accounts.map((account) => (
              <SelectItem
                key={account.id}
                startContent={
                  <span
                    className="h-3 w-3 rounded-full"
                    style={{ backgroundColor: account.color }}
                  />
                }
              >
                {account.label}
              </SelectItem>
            ))}
          </Select>

          <Select
            size="sm"
            label={t('repository.githubAccount')}
            aria-label={t('repository.githubAccount')}
            placeholder={t('repository.unassigned')}
            selectedKeys={repo.ghUsername ? [repo.ghUsername] : []}
            isDisabled={!ghAccounts.data || ghAccounts.data.length === 0}
            onSelectionChange={(keys) => {
              const username = firstSelectedKey(keys);
              if (username) update(repo.id, { ghUsername: username });
            }}
          >
            {(ghAccounts.data ?? []).map((account) => (
              <SelectItem key={account.username}>{account.username}</SelectItem>
            ))}
          </Select>
        </div>

        <div className="flex items-center gap-2">
          <Dropdown>
            <DropdownTrigger>
              <Button
                size="sm"
                variant="flat"
                startContent={<FolderOpen size={15} />}
                endContent={<ChevronDown size={14} />}
              >
                {t('launcher.open')}
              </Button>
            </DropdownTrigger>
            <DropdownMenu
              aria-label={t('launcher.open')}
              items={openItems}
              onAction={(key) => void handleOpen(String(key))}
            >
              {(item) => <DropdownItem key={item.key}>{item.label}</DropdownItem>}
            </DropdownMenu>
          </Dropdown>

          <p className="min-h-4 flex-1 truncate text-xs text-danger">
            {openError ?? (sync.isError ? (sync.error as Error).message : '')}
            {sync.data && !sync.data.overallSuccess && failureMessage(sync.data)}
          </p>

          <Button
            size="sm"
            color="primary"
            startContent={<ArrowRightLeft size={14} />}
            isDisabled={!assignedAccount}
            isLoading={sync.isPending}
            onPress={() => sync.mutate()}
          >
            {t('repository.sync')}
          </Button>
        </div>
      </CardBody>
    </Card>
  );
}
