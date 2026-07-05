import { FolderGit2, Users, Terminal, ScrollText, Settings, type LucideIcon } from 'lucide-react';

export interface NavItem {
  to: string;
  /** i18n key under `nav.*` */
  labelKey: string;
  icon: LucideIcon;
}

export const NAV_ITEMS: NavItem[] = [
  { to: '/projects', labelKey: 'nav.projects', icon: FolderGit2 },
  { to: '/git-accounts', labelKey: 'nav.gitAccounts', icon: Users },
  { to: '/github-cli', labelKey: 'nav.githubCli', icon: Terminal },
  { to: '/logs', labelKey: 'nav.logs', icon: ScrollText },
  { to: '/settings', labelKey: 'nav.settings', icon: Settings },
];
