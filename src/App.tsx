import { HeroUIProvider } from '@heroui/react';
import { useHref, useNavigate } from 'react-router-dom';

import { AppShell } from '@/components/layout/AppShell';
import { AppRoutes } from '@/routes';

export default function App() {
  const navigate = useNavigate();

  return (
    <HeroUIProvider navigate={navigate} useHref={useHref}>
      <AppShell>
        <AppRoutes />
      </AppShell>
    </HeroUIProvider>
  );
}
