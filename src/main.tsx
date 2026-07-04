import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { QueryClientProvider } from '@tanstack/react-query';

import App from './App';
import { queryClient } from '@/lib/queryClient';
import { useSettingsStore } from '@/stores/settingsStore';
import { useRepositoriesStore } from '@/features/repositories/store';
import { useGitAccountsStore } from '@/features/git-accounts/store';
import { useLogsStore } from '@/features/logs/store';
import '@/lib/i18n';
import '@/styles/global.css';

const rootElement = document.getElementById('root');
if (!rootElement) throw new Error('Root element #root not found');

async function bootstrap() {
  // Load persisted state before the first paint (theme/language avoid a flash).
  await Promise.all([
    useSettingsStore.getState().hydrate(),
    useRepositoriesStore.getState().hydrate(),
    useGitAccountsStore.getState().hydrate(),
    useLogsStore.getState().hydrate(),
  ]);

  ReactDOM.createRoot(rootElement!).render(
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </QueryClientProvider>
    </React.StrictMode>,
  );
}

void bootstrap();
