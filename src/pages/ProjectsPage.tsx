import { useState } from 'react';
import { Button, ButtonGroup, Input, Select, SelectItem, Switch, Tooltip } from '@heroui/react';
import { FolderPlus, LayoutGrid, List, Search } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import { firstSelectedKey } from '@/lib/selection';
import { RepositoryCard } from '@/features/repositories/components/RepositoryCard';
import { filterAndSortRepos, type RepoSort } from '@/features/repositories/filtering';
import { deriveRepoName } from '@/features/repositories/naming';
import { useRepositoriesStore } from '@/features/repositories/store';
import { useGitAccountsStore } from '@/features/git-accounts/store';
import { useSettingsStore } from '@/stores/settingsStore';
import { useUiStore } from '@/stores/uiStore';
import { pickDirectory } from '@/services/dialog.service';
import { gitService } from '@/services/git.service';

export function ProjectsPage() {
  const { t } = useTranslation();
  const repositories = useRepositoriesStore((s) => s.repositories);
  const add = useRepositoriesStore((s) => s.add);
  const remove = useRepositoriesStore((s) => s.remove);
  const accounts = useGitAccountsStore((s) => s.accounts);
  const viewMode = useSettingsStore((s) => s.viewMode);
  const setViewMode = useSettingsStore((s) => s.setViewMode);
  const ui = useUiStore();

  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const visible = filterAndSortRepos(repositories, {
    search: ui.search,
    sortBy: ui.sortBy,
    gitAccountId: ui.filterGitAccountId,
    favoritesOnly: ui.favoritesOnly,
  });

  async function handleAdd() {
    setError(null);
    setBusy(true);
    try {
      const path = await pickDirectory(t('repository.pickTitle'));
      if (!path) return;
      if (repositories.some((r) => r.path === path)) {
        setError(t('repository.alreadyAdded'));
        return;
      }
      if (!(await gitService.isRepo(path))) {
        setError(t('repository.notARepo'));
        return;
      }
      add({
        id: crypto.randomUUID(),
        name: deriveRepoName(path),
        path,
        addedAt: new Date().toISOString(),
      });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <Page
      title={t('nav.projects')}
      actions={
        <Button
          color="primary"
          startContent={<FolderPlus size={16} />}
          onPress={handleAdd}
          isLoading={busy}
        >
          {t('repository.add')}
        </Button>
      }
    >
      {error && <p className="text-sm text-danger">{error}</p>}

      {repositories.length === 0 ? (
        <p className="text-sm text-default-500">{t('repository.emptyHint')}</p>
      ) : (
        <>
          <div className="flex flex-wrap items-center gap-2">
            <Input
              size="sm"
              className="max-w-xs"
              placeholder={t('common.search')}
              value={ui.search}
              onValueChange={ui.setSearch}
              startContent={<Search size={15} className="text-default-400" />}
              isClearable
              onClear={() => ui.setSearch('')}
            />
            <Select
              size="sm"
              className="w-36"
              aria-label={t('repository.sortBy')}
              selectedKeys={[ui.sortBy]}
              onSelectionChange={(keys) => {
                const value = firstSelectedKey<RepoSort>(keys);
                if (value) ui.setSortBy(value);
              }}
            >
              <SelectItem key="name">{t('repository.sortName')}</SelectItem>
              <SelectItem key="recent">{t('repository.sortRecent')}</SelectItem>
            </Select>
            <Select
              size="sm"
              className="w-44"
              aria-label={t('repository.filterByAccount')}
              selectedKeys={[ui.filterGitAccountId ?? 'all']}
              onSelectionChange={(keys) => {
                const value = firstSelectedKey<string>(keys);
                ui.setFilterGitAccountId(value && value !== 'all' ? value : null);
              }}
            >
              {[
                <SelectItem key="all">{t('repository.allAccounts')}</SelectItem>,
                ...accounts.map((a) => <SelectItem key={a.id}>{a.label}</SelectItem>),
              ]}
            </Select>
            <Switch size="sm" isSelected={ui.favoritesOnly} onValueChange={ui.setFavoritesOnly}>
              <span className="text-sm">{t('repository.favoritesOnly')}</span>
            </Switch>

            <ButtonGroup size="sm" variant="flat" className="ml-auto">
              <Tooltip content={t('repository.gridView')} closeDelay={0}>
                <Button
                  isIconOnly
                  aria-label={t('repository.gridView')}
                  color={viewMode === 'grid' ? 'primary' : 'default'}
                  onPress={() => setViewMode('grid')}
                >
                  <LayoutGrid size={16} />
                </Button>
              </Tooltip>
              <Tooltip content={t('repository.listView')} closeDelay={0}>
                <Button
                  isIconOnly
                  aria-label={t('repository.listView')}
                  color={viewMode === 'list' ? 'primary' : 'default'}
                  onPress={() => setViewMode('list')}
                >
                  <List size={16} />
                </Button>
              </Tooltip>
            </ButtonGroup>
          </div>

          {visible.length === 0 ? (
            <p className="text-sm text-default-500">{t('repository.noMatches')}</p>
          ) : (
            <div
              className={
                viewMode === 'grid'
                  ? 'grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3'
                  : 'flex flex-col gap-3'
              }
            >
              {visible.map((repo) => (
                <RepositoryCard key={repo.id} repo={repo} onRemove={remove} />
              ))}
            </div>
          )}
        </>
      )}
    </Page>
  );
}
