import type { ReactNode } from 'react';

interface PageProps {
  title: string;
  description?: string;
  actions?: ReactNode;
  children: ReactNode;
}

/** Consistent page frame: a header row (title + optional actions) and content. */
export function Page({ title, description, actions, children }: PageProps) {
  return (
    <div className="mx-auto flex max-w-5xl flex-col gap-6 px-8 py-8">
      <header className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-xl font-semibold tracking-tight">{title}</h1>
          {description && <p className="mt-1 text-sm text-default-500">{description}</p>}
        </div>
        {actions && <div className="flex shrink-0 items-center gap-2">{actions}</div>}
      </header>
      {children}
    </div>
  );
}
