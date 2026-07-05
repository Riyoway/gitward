import { useQuery } from '@tanstack/react-query';
import { Button, Card, CardBody, Chip, Select, SelectItem, Spinner, Switch } from '@heroui/react';
import { Check, Download, RefreshCw, Upload, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';

import { Page } from '@/components/layout/Page';
import type { Language } from '@/lib/i18n';
import { queryKeys } from '@/lib/queryKeys';
import { firstSelectedKey } from '@/lib/selection';
import type { Theme } from '@/lib/theme';
import { toastError, toastSuccess } from '@/lib/toast';
import { applyBackup, buildBackup, parseBackup } from '@/features/backup/backup';
import { pickJsonFile, pickSavePath } from '@/services/dialog.service';
import { fsService } from '@/services/fs.service';
import { healthService } from '@/services/health.service';
import { launcherService } from '@/services/launcher.service';
import { useSettingsStore } from '@/stores/settingsStore';

function HealthRow({ label, ok }: { label: string; ok: boolean }) {
  return (
    <div className="flex items-center gap-2 text-sm">
      {ok ? (
        <Check size={16} className="text-success" />
      ) : (
        <X size={16} className="text-danger" />
      )}
      <span className={ok ? '' : 'text-default-500'}>{label}</span>
    </div>
  );
}

const THEMES: Theme[] = ['system', 'light', 'dark'];
const LANGUAGES: { key: Language; label: string }[] = [
  { key: 'ja', label: '日本語' },
  { key: 'en', label: 'English' },
];

export function SettingsPage() {
  const { t } = useTranslation();
  const { theme, language, autoSwitch, terminalId, setTheme, setLanguage, setAutoSwitch, setTerminalId } =
    useSettingsStore();
  const health = useQuery({ queryKey: queryKeys.health, queryFn: healthService.check });
  // Separate from health so the tool/terminal lists show without waiting on the
  // network probe inside the health check.
  const tools = useQuery({ queryKey: queryKeys.tools, queryFn: launcherService.detectTools });
  const installedTools = (tools.data ?? []).filter((tool) => tool.installed);
  const installedTerminals = installedTools.filter((tool) => tool.category === 'terminal');

  async function handleExport() {
    try {
      const path = await pickSavePath('gitward-backup.json');
      if (!path) return;
      await fsService.writeTextFile(path, JSON.stringify(buildBackup(), null, 2));
      toastSuccess(t('settings.exportDone'));
    } catch (e) {
      toastError(t('settings.exportFailed'), e instanceof Error ? e.message : String(e));
    }
  }

  async function handleImport() {
    try {
      const path = await pickJsonFile(t('settings.importTitle'));
      if (!path) return;
      applyBackup(parseBackup(await fsService.readTextFile(path)));
      toastSuccess(t('settings.importDone'));
    } catch (e) {
      toastError(t('settings.importFailed'), e instanceof Error ? e.message : String(e));
    }
  }

  const themeLabels: Record<Theme, string> = {
    system: t('settings.themeSystem'),
    light: t('settings.themeLight'),
    dark: t('settings.themeDark'),
  };

  return (
    <Page title={t('nav.settings')}>
      <Card shadow="sm">
        <CardBody className="gap-5 p-6">
          <h2 className="text-sm font-medium text-default-500">{t('settings.appearance')}</h2>

          <Select
            label={t('settings.theme')}
            selectedKeys={[theme]}
            onSelectionChange={(keys) => {
              const value = firstSelectedKey<Theme>(keys);
              if (value) setTheme(value);
            }}
            className="max-w-xs"
          >
            {THEMES.map((value) => (
              <SelectItem key={value}>{themeLabels[value]}</SelectItem>
            ))}
          </Select>

          <Select
            label={t('settings.language')}
            selectedKeys={[language]}
            onSelectionChange={(keys) => {
              const value = firstSelectedKey<Language>(keys);
              if (value) setLanguage(value);
            }}
            className="max-w-xs"
          >
            {LANGUAGES.map(({ key, label }) => (
              <SelectItem key={key}>{label}</SelectItem>
            ))}
          </Select>
        </CardBody>
      </Card>

      <Card shadow="sm">
        <CardBody className="gap-5 p-6">
          <h2 className="text-sm font-medium text-default-500">{t('settings.behavior')}</h2>
          <Switch isSelected={autoSwitch} onValueChange={setAutoSwitch}>
            <div className="flex flex-col">
              <span className="text-sm">{t('settings.autoSwitch')}</span>
              <span className="text-xs text-default-400">{t('settings.autoSwitchDesc')}</span>
            </div>
          </Switch>

          <Select
            label={t('settings.terminal')}
            description={t('settings.terminalDesc')}
            selectedKeys={[terminalId || 'default']}
            onSelectionChange={(keys) => {
              const value = firstSelectedKey<string>(keys);
              setTerminalId(value === 'default' ? '' : (value ?? ''));
            }}
            className="max-w-xs"
          >
            {[
              <SelectItem key="default">{t('settings.terminalDefault')}</SelectItem>,
              ...installedTerminals.map((term) => <SelectItem key={term.id}>{term.name}</SelectItem>),
            ]}
          </Select>
        </CardBody>
      </Card>

      <Card shadow="sm">
        <CardBody className="gap-4 p-6">
          <div className="flex items-center justify-between">
            <h2 className="text-sm font-medium text-default-500">{t('settings.diagnostics')}</h2>
            <Button
              size="sm"
              isIconOnly
              variant="light"
              aria-label={t('common.refresh')}
              onPress={() => health.refetch()}
            >
              <RefreshCw size={15} className={health.isFetching ? 'animate-spin' : ''} />
            </Button>
          </div>

          {health.isPending ? (
            <Spinner size="sm" />
          ) : health.isError ? (
            <p className="text-sm text-danger">{t('settings.healthUnavailable')}</p>
          ) : (
            <>
              <div className="grid grid-cols-2 gap-2">
                <HealthRow label="git" ok={health.data.git} />
                <HealthRow label="gh" ok={health.data.gh} />
                <HealthRow label="ssh" ok={health.data.ssh} />
                <HealthRow label={t('settings.internet')} ok={health.data.internet} />
              </div>
              <div>
                <p className="mb-2 text-xs text-default-500">{t('settings.detectedTools')}</p>
                {installedTools.length > 0 ? (
                  <div className="flex flex-wrap gap-2">
                    {installedTools.map((tool) => (
                      <Chip key={tool.id} size="sm" variant="flat">
                        {tool.name}
                      </Chip>
                    ))}
                  </div>
                ) : (
                  <p className="text-xs text-default-400">{t('settings.toolsNone')}</p>
                )}
              </div>
            </>
          )}
        </CardBody>
      </Card>

      <Card shadow="sm">
        <CardBody className="gap-4 p-6">
          <h2 className="text-sm font-medium text-default-500">{t('settings.data')}</h2>
          <p className="text-xs text-default-400">{t('settings.dataDesc')}</p>
          <div className="flex gap-2">
            <Button variant="flat" startContent={<Download size={16} />} onPress={handleExport}>
              {t('settings.export')}
            </Button>
            <Button variant="flat" startContent={<Upload size={16} />} onPress={handleImport}>
              {t('settings.import')}
            </Button>
          </div>
        </CardBody>
      </Card>
    </Page>
  );
}
