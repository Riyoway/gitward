import { QueryClient } from '@tanstack/react-query';

/**
 * External state (git status, gh accounts, tool detection) is read through
 * TanStack Query. Results reflect the machine's current state, so we keep them
 * fresh-ish but avoid hammering external processes: no refetch on window focus,
 * a short stale window, and one retry for transient command failures.
 */
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30_000,
      gcTime: 5 * 60_000,
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});
