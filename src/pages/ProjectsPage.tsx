import { useState } from 'react';
import { Button } from '@heroui/react';
import { FolderPlus } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import { RepositoryCard } from '@/features/repositories/components/RepositoryCard';
import { deriveRepoName } from '@/features/repositories/naming';
import { useRepositoriesStore } from '@/features/repositories/store';
import { pickDirectory } from '@/services/dialog.service';
import { gitService } from '@/services/git.service';

export function ProjectsPage() {
  const { t } = useTranslation();
  const repositories = useRepositoriesStore((s) => s.repositories);
  const add = useRepositoriesStore((s) => s.add);
  const remove = useRepositoriesStore((s) => s.remove);

  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

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
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {repositories.map((repo) => (
            <RepositoryCard key={repo.id} repo={repo} onRemove={remove} />
          ))}
        </div>
      )}
    </Page>
  );
}
