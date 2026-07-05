import { NavLink } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { GitBranch } from 'lucide-react';

import { NAV_ITEMS, type NavItem } from './nav';

function NavRow({ item, t }: { item: NavItem; t: (key: string) => string }) {
  const { to, labelKey, icon: Icon } = item;
  return (
    <NavLink
      to={to}
      className={({ isActive }) =>
        [
          'flex items-center gap-3 rounded-medium px-3 py-2 text-sm transition-colors',
          isActive
            ? 'bg-primary/10 font-medium text-primary'
            : 'text-default-600 hover:bg-default-100 hover:text-foreground',
        ].join(' ')
      }
    >
      <Icon size={18} />
      {t(labelKey)}
    </NavLink>
  );
}

export function Sidebar() {
  const { t } = useTranslation();

  // Settings sits pinned at the bottom; everything else at the top.
  const settings = NAV_ITEMS.find((i) => i.to === '/settings');
  const primary = NAV_ITEMS.filter((i) => i.to !== '/settings');

  return (
    <aside className="flex h-full w-56 flex-col border-r border-divider bg-content1">
      <div className="flex items-center gap-2 px-5 py-5">
        <GitBranch className="text-primary" size={22} />
        <span className="text-lg font-semibold tracking-tight">{t('app.name')}</span>
      </div>

      <nav className="flex flex-1 flex-col gap-1 px-3">
        {primary.map((item) => (
          <NavRow key={item.to} item={item} t={t} />
        ))}
      </nav>

      {settings && (
        <nav className="flex flex-col gap-1 border-t border-divider px-3 py-3">
          <NavRow item={settings} t={t} />
        </nav>
      )}
    </aside>
  );
}
