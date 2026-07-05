import { Navigate, Route, Routes } from 'react-router-dom';

import { ProjectsPage } from '@/pages/ProjectsPage';
import { GitAccountsPage } from '@/pages/GitAccountsPage';
import { GitHubCliPage } from '@/pages/GitHubCliPage';
import { LogsPage } from '@/pages/LogsPage';
import { SettingsPage } from '@/pages/SettingsPage';

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<Navigate to="/projects" replace />} />
      <Route path="/projects" element={<ProjectsPage />} />
      <Route path="/git-accounts" element={<GitAccountsPage />} />
      <Route path="/github-cli" element={<GitHubCliPage />} />
      <Route path="/logs" element={<LogsPage />} />
      <Route path="/settings" element={<SettingsPage />} />
      <Route path="*" element={<Navigate to="/projects" replace />} />
    </Routes>
  );
}
