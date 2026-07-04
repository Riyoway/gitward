import { useState } from 'react';
import { Button, Card, CardBody, Chip, Input, Spinner } from '@heroui/react';
import { FolderOpen, Save } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import { toastError, toastSuccess } from '@/lib/toast';
import { pickDirectory } from '@/services/dialog.service';
import { identityService } from '@/services/identity.service';
import type { IdentityField } from '@/types';

export function AppIdentityPage() {
  const { t } = useTranslation();
  const [path, setPath] = useState<string | null>(null);
  const [frameworks, setFrameworks] = useState<string[]>([]);
  const [fields, setFields] = useState<IdentityField[]>([]);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);

  async function load(target: string) {
    setLoading(true);
    try {
      const [detected, read] = await Promise.all([
        identityService.detect(target),
        identityService.read(target),
      ]);
      setFrameworks(detected);
      setFields(read);
    } catch (e) {
      toastError(t('appIdentity.loadFailed'), e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }

  async function choose() {
    const target = await pickDirectory(t('appIdentity.pickTitle'));
    if (!target) return;
    setPath(target);
    await load(target);
  }

  async function save() {
    if (!path) return;
    setSaving(true);
    try {
      const payload = fields
        .filter((f) => (f.value ?? '').trim() !== '')
        .map((f) => ({ id: f.id, value: (f.value ?? '').trim() }));
      await identityService.write(path, payload);
      toastSuccess(t('appIdentity.saved'));
      await load(path);
    } catch (e) {
      toastError(t('appIdentity.saveFailed'), e instanceof Error ? e.message : String(e));
    } finally {
      setSaving(false);
    }
  }

  const setValue = (id: string, value: string) =>
    setFields((current) => current.map((f) => (f.id === id ? { ...f, value } : f)));

  return (
    <Page
      title={t('nav.appIdentity')}
      actions={
        <Button variant="flat" startContent={<FolderOpen size={16} />} onPress={choose}>
          {t('appIdentity.choose')}
        </Button>
      }
    >
      {!path ? (
        <p className="text-sm text-default-500">{t('appIdentity.hint')}</p>
      ) : loading ? (
        <Spinner />
      ) : (
        <>
          <div className="flex flex-wrap items-center gap-2">
            {frameworks.length > 0 ? (
              frameworks.map((fw) => (
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
          <p className="truncate font-mono text-xs text-default-400" title={path}>
            {path}
          </p>

          {fields.length === 0 ? (
            <p className="text-sm text-default-500">{t('appIdentity.noFields')}</p>
          ) : (
            <>
              <Card shadow="sm">
                <CardBody className="gap-4 p-6">
                  {fields.map((field) => (
                    <Input
                      key={field.id}
                      label={field.label}
                      description={field.file}
                      value={field.value ?? ''}
                      onValueChange={(v) => setValue(field.id, v)}
                    />
                  ))}
                </CardBody>
              </Card>
              <div className="flex items-center gap-3">
                <Button
                  color="primary"
                  startContent={<Save size={16} />}
                  onPress={save}
                  isLoading={saving}
                >
                  {t('common.save')}
                </Button>
                <span className="text-xs text-default-400">{t('appIdentity.backupNote')}</span>
              </div>
            </>
          )}
        </>
      )}
    </Page>
  );
}
