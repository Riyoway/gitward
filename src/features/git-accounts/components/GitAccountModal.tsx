import { useState } from 'react';
import {
  Button,
  Input,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from '@heroui/react';
import { Check } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { ACCOUNT_COLORS, DEFAULT_ACCOUNT_COLOR } from '../colors';
import type { GitAccount, GitAccountDraft } from '../types';
import { validateGitAccount, type GitAccountErrors } from '../validation';

interface GitAccountModalProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  /** When set, the modal edits this account; otherwise it creates a new one. */
  editing: GitAccount | null;
  onSubmit: (draft: GitAccountDraft) => void;
}

function draftFor(editing: GitAccount | null): GitAccountDraft {
  return editing
    ? { label: editing.label, userName: editing.userName, email: editing.email, color: editing.color }
    : { label: '', userName: '', email: '', color: DEFAULT_ACCOUNT_COLOR };
}

export function GitAccountModal({ isOpen, onOpenChange, editing, onSubmit }: GitAccountModalProps) {
  return (
    <Modal isOpen={isOpen} onOpenChange={onOpenChange} placement="center">
      <ModalContent>
        {(onClose) => (
          // Keyed by the target so switching accounts (or edit vs. add) remounts
          // the form with fresh state instead of resetting via an effect.
          <GitAccountForm
            key={editing?.id ?? 'new'}
            editing={editing}
            onClose={onClose}
            onSubmit={onSubmit}
          />
        )}
      </ModalContent>
    </Modal>
  );
}

interface GitAccountFormProps {
  editing: GitAccount | null;
  onClose: () => void;
  onSubmit: (draft: GitAccountDraft) => void;
}

function GitAccountForm({ editing, onClose, onSubmit }: GitAccountFormProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<GitAccountDraft>(() => draftFor(editing));
  const [errors, setErrors] = useState<GitAccountErrors>({});

  const set = (patch: Partial<GitAccountDraft>) => setDraft((d) => ({ ...d, ...patch }));
  const errorText = (field: keyof GitAccountErrors) =>
    errors[field] ? t(`gitAccount.error.${errors[field]}`) : undefined;

  const handleSave = () => {
    const found = validateGitAccount(draft);
    if (Object.keys(found).length > 0) {
      setErrors(found);
      return;
    }
    onSubmit({
      ...draft,
      label: draft.label.trim(),
      userName: draft.userName.trim(),
      email: draft.email.trim(),
    });
    onClose();
  };

  return (
    <>
      <ModalHeader>{editing ? t('gitAccount.edit') : t('gitAccount.add')}</ModalHeader>
      <ModalBody className="gap-4">
        <Input
          label={t('gitAccount.label')}
          placeholder={t('gitAccount.labelPlaceholder')}
          value={draft.label}
          onValueChange={(v) => set({ label: v })}
          isInvalid={!!errors.label}
          errorMessage={errorText('label')}
          autoFocus
        />
        <Input
          label={t('gitAccount.userName')}
          value={draft.userName}
          onValueChange={(v) => set({ userName: v })}
          isInvalid={!!errors.userName}
          errorMessage={errorText('userName')}
        />
        <Input
          label={t('gitAccount.email')}
          type="email"
          value={draft.email}
          onValueChange={(v) => set({ email: v })}
          isInvalid={!!errors.email}
          errorMessage={errorText('email')}
        />
        <div>
          <p className="mb-2 text-sm text-default-600">{t('gitAccount.color')}</p>
          <div className="flex flex-wrap gap-2">
            {ACCOUNT_COLORS.map((color) => (
              <button
                key={color}
                type="button"
                aria-label={color}
                onClick={() => set({ color })}
                className="flex h-7 w-7 items-center justify-center rounded-full transition-transform hover:scale-110"
                style={{ backgroundColor: color }}
              >
                {draft.color === color && <Check size={15} className="text-white" />}
              </button>
            ))}
          </div>
        </div>
      </ModalBody>
      <ModalFooter>
        <Button variant="light" onPress={onClose}>
          {t('common.cancel')}
        </Button>
        <Button color="primary" onPress={handleSave}>
          {t('common.save')}
        </Button>
      </ModalFooter>
    </>
  );
}
