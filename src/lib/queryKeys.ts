/** Central registry of TanStack Query keys, so invalidation stays consistent. */
export const queryKeys = {
  repoStatus: (id: string) => ['repo-status', id] as const,
  repoIdentity: (id: string) => ['repo-identity', id] as const,
};
