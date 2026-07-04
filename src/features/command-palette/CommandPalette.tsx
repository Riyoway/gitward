import { useMemo, useState } from 'react';
import { Input, Listbox, ListboxItem, Modal, ModalContent } from '@heroui/react';
import { Search } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

import { NAV_ITEMS } from '@/components/layout/nav';
import type { Theme } from '@/lib/theme';
import { useRepositoriesStore } from '@/features/repositories/store';
import { useSettingsStore } from '@/stores/settingsStore';
import { useUiStore } from '@/stores/uiStore';

interface Command {
  id: string;
  label: string;
  section: string;
  run: () => void;
}

function nextTheme(theme: Theme): Theme {
  return theme === 'system' ? 'light' : theme === 'light' ? 'dark' : 'system';
}

export function CommandPalette({ isOpen, onClose }: { isOpen: boolean; onClose: () => void }) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const repos = useRepositoriesStore((s) => s.repositories);
  const setSearch = useUiStore((s) => s.setSearch);
  const { theme, language, setTheme, setLanguage } = useSettingsStore();
  const [query, setQuery] = useState('');

  const commands = useMemo<Command[]>(() => {
    const nav = NAV_ITEMS.map((item) => ({
      id: `nav:${item.to}`,
      section: t('palette.navigate'),
      label: t(item.labelKey),
      run: () => navigate(item.to),
    }));
    const actions: Command[] = [
      {
        id: 'act:theme',
        section: t('palette.actions'),
        label: t('palette.cycleTheme'),
        run: () => setTheme(nextTheme(theme)),
      },
      {
        id: 'act:lang',
        section: t('palette.actions'),
        label: t('palette.toggleLanguage'),
        run: () => setLanguage(language === 'ja' ? 'en' : 'ja'),
      },
    ];
    const repoCmds = repos.map((r) => ({
      id: `repo:${r.id}`,
      section: t('palette.repositories'),
      label: r.name,
      run: () => {
        setSearch(r.name);
        navigate('/projects');
      },
    }));
    return [...nav, ...actions, ...repoCmds];
  }, [t, navigate, repos, setSearch, theme, language, setTheme, setLanguage]);

  const q = query.trim().toLowerCase();
  const filtered = q ? commands.filter((c) => c.label.toLowerCase().includes(q)) : commands;

  const close = () => {
    setQuery('');
    onClose();
  };
  const run = (id: string) => {
    commands.find((c) => c.id === id)?.run();
    close();
  };

  return (
    <Modal
      isOpen={isOpen}
      onOpenChange={(open) => !open && close()}
      placement="top"
      size="lg"
      hideCloseButton
    >
      <ModalContent>
        <div className="flex flex-col gap-2 p-2">
          <Input
            autoFocus
            variant="bordered"
            placeholder={t('palette.placeholder')}
            value={query}
            onValueChange={setQuery}
            startContent={<Search size={16} className="text-default-400" />}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && filtered[0]) run(filtered[0].id);
            }}
          />
          <div className="max-h-80 overflow-y-auto">
            <Listbox
              aria-label={t('palette.title')}
              emptyContent={t('palette.noResults')}
              onAction={(key) => run(String(key))}
            >
              {filtered.map((c) => (
                <ListboxItem key={c.id} description={c.section}>
                  {c.label}
                </ListboxItem>
              ))}
            </Listbox>
          </div>
        </div>
      </ModalContent>
    </Modal>
  );
}
