import { NavLink } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { GitBranch } from 'lucide-react';

import { NAV_ITEMS } from './nav';

export function Sidebar() {
  const { t } = useTranslation();

  return (
    <aside className="flex h-full w-56 flex-col border-r border-divider bg-content1">
      <div className="flex items-center gap-2 px-5 py-5">
        <GitBranch className="text-primary" size={22} />
        <span className="text-lg font-semibold tracking-tight">{t('app.name')}</span>
      </div>

      <nav className="flex flex-1 flex-col gap-1 px-3">
        {NAV_ITEMS.map(({ to, labelKey, icon: Icon }) => (
          <NavLink
            key={to}
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
        ))}
      </nav>
    </aside>
  );
}
