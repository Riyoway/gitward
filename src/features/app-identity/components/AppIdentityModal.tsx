import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  Button,
  Chip,
  Input,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  Spinner,
} from '@heroui/react';
import { Save } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { queryKeys } from '@/lib/queryKeys';
import { toastError, toastSuccess } from '@/lib/toast';
import { identityService } from '@/services/identity.service';
import type { Repository } from '@/features/repositories/types';

interface AppIdentityModalProps {
  repo: Repository;
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
}

export function AppIdentityModal({ repo, isOpen, onOpenChange }: AppIdentityModalProps) {
  return (
    <Modal
      isOpen={isOpen}
      onOpenChange={onOpenChange}
      size="lg"
      placement="center"
      scrollBehavior="inside"
    >
      <ModalContent>
        {(onClose) => <AppIdentityForm repo={repo} onClose={onClose} />}
      </ModalContent>
    </Modal>
  );
}

function AppIdentityForm({ repo, onClose }: { repo: Repository; onClose: () => void }) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: queryKeys.appIdentity(repo.path),
    queryFn: async () => ({
      frameworks: await identityService.detect(repo.path),
      fields: await identityService.read(repo.path),
    }),
  });

  // Overlay of user edits; effective value = edit ?? loaded value. Avoids
  // seeding state from async data in an effect.
  const [edits, setEdits] = useState<Record<string, string>>({});

  const save = useMutation({
    mutationFn: () => {
      const payload = (query.data?.fields ?? [])
        .map((f) => ({ id: f.id, value: (edits[f.id] ?? f.value ?? '').trim() }))
        .filter((p) => p.value !== '');
      return identityService.write(repo.path, payload);
    },
    onSuccess: () => {
      toastSuccess(t('appIdentity.saved'));
      setEdits({});
      void queryClient.invalidateQueries({ queryKey: queryKeys.appIdentity(repo.path) });
    },
    onError: (e) => toastError(t('appIdentity.saveFailed'), (e as Error).message),
  });

  return (
    <>
      <ModalHeader className="flex flex-col gap-1">
        <span>{t('appIdentity.title')}</span>
        <span className="truncate font-mono text-xs font-normal text-default-400">{repo.name}</span>
      </ModalHeader>
      <ModalBody className="gap-4">
        {query.isPending ? (
          <Spinner />
        ) : query.isError ? (
          <p className="text-sm text-danger">{t('appIdentity.loadFailed')}</p>
        ) : (
          <>
            <div className="flex flex-wrap items-center gap-2">
              {query.data.frameworks.length > 0 ? (
                query.data.frameworks.map((fw) => (
                  <Chip key={fw} size="sm" variant="flat" color="primary">
                    {fw}
                  </Chip>
                ))
              ) : (
                <Chip size="sm" variant="flat">
                  {t('appIdentity.noFramework')}
                </Chip>
              )}
            </div>

            {query.data.fields.length === 0 ? (
              <p className="text-sm text-default-500">{t('appIdentity.noFields')}</p>
            ) : (
              query.data.fields.map((field) => (
                <Input
                  key={field.id}
                  label={field.label}
                  description={field.file}
                  value={edits[field.id] ?? field.value ?? ''}
                  onValueChange={(v) => setEdits((e) => ({ ...e, [field.id]: v }))}
                />
              ))
            )}
          </>
        )}
      </ModalBody>
      <ModalFooter className="items-center">
        <span className="mr-auto text-xs text-default-400">{t('appIdentity.backupNote')}</span>
        <Button variant="light" onPress={onClose}>
          {t('common.cancel')}
        </Button>
        <Button
          color="primary"
          startContent={<Save size={16} />}
          onPress={() => save.mutate()}
          isLoading={save.isPending}
          isDisabled={!query.data || query.data.fields.length === 0}
        >
          {t('common.save')}
        </Button>
      </ModalFooter>
    </>
  );
}
