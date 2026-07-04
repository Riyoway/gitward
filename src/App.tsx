import { useEffect, useState } from 'react';
import { HeroUIProvider, ToastProvider } from '@heroui/react';
import { useHref, useNavigate } from 'react-router-dom';

import { AppShell } from '@/components/layout/AppShell';
import { CommandPalette } from '@/features/command-palette/CommandPalette';
import { AppRoutes } from '@/routes';

export default function App() {
  const navigate = useNavigate();
  const [paletteOpen, setPaletteOpen] = useState(false);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setPaletteOpen((open) => !open);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);

  return (
    <HeroUIProvider navigate={navigate} useHref={useHref}>
      <ToastProvider placement="bottom-right" />
      <AppShell>
        <AppRoutes />
      </AppShell>
      <CommandPalette isOpen={paletteOpen} onClose={() => setPaletteOpen(false)} />
    </HeroUIProvider>
  );
}
