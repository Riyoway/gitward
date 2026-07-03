import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { QueryClientProvider } from '@tanstack/react-query';

import App from './App';
import { queryClient } from '@/lib/queryClient';
import { useSettingsStore } from '@/stores/settingsStore';
import '@/lib/i18n';
import '@/styles/global.css';

const rootElement = document.getElementById('root');
if (!rootElement) throw new Error('Root element #root not found');

async function bootstrap() {
  // Apply persisted theme/language before the first paint to avoid a flash.
  await useSettingsStore.getState().hydrate();

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
