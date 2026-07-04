import { HeroUIProvider, ToastProvider } from '@heroui/react';
import { useHref, useNavigate } from 'react-router-dom';

import { AppShell } from '@/components/layout/AppShell';
import { AppRoutes } from '@/routes';

export default function App() {
  const navigate = useNavigate();

  return (
    <HeroUIProvider navigate={navigate} useHref={useHref}>
      <ToastProvider placement="bottom-right" />
      <AppShell>
        <AppRoutes />
      </AppShell>
    </HeroUIProvider>
  );
}
