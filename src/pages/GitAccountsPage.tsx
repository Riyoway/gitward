import { useState } from 'react';
import { Button, useDisclosure } from '@heroui/react';
import { UserPlus } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import { GitAccountCard } from '@/features/git-accounts/components/GitAccountCard';
import { GitAccountModal } from '@/features/git-accounts/components/GitAccountModal';
import { useGitAccountsStore } from '@/features/git-accounts/store';
import type { GitAccount, GitAccountDraft } from '@/features/git-accounts/types';

export function GitAccountsPage() {
  const { t } = useTranslation();
  const accounts = useGitAccountsStore((s) => s.accounts);
  const add = useGitAccountsStore((s) => s.add);
  const update = useGitAccountsStore((s) => s.update);
  const remove = useGitAccountsStore((s) => s.remove);

  const { isOpen, onOpen, onOpenChange } = useDisclosure();
  const [editing, setEditing] = useState<GitAccount | null>(null);

  const openAdd = () => {
    setEditing(null);
    onOpen();
  };
  const openEdit = (account: GitAccount) => {
    setEditing(account);
    onOpen();
  };
  const handleSubmit = (draft: GitAccountDraft) => {
    if (editing) update(editing.id, draft);
    else add(draft);
  };

  return (
    <Page
      title={t('nav.gitAccounts')}
      actions={
        <Button color="primary" startContent={<UserPlus size={16} />} onPress={openAdd}>
          {t('gitAccount.add')}
        </Button>
      }
    >
      {accounts.length === 0 ? (
        <p className="text-sm text-default-500">{t('gitAccount.emptyHint')}</p>
      ) : (
        <div className="flex flex-col gap-3">
          {accounts.map((account) => (
            <GitAccountCard
              key={account.id}
              account={account}
              onEdit={openEdit}
              onRemove={remove}
            />
          ))}
        </div>
      )}

      <GitAccountModal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        editing={editing}
        onSubmit={handleSubmit}
      />
    </Page>
  );
}
