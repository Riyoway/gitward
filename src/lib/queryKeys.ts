/** Central registry of TanStack Query keys, so invalidation stays consistent. */
export const queryKeys = {
  repoStatus: (id: string) => ['repo-status', id] as const,
  repoIdentity: (id: string) => ['repo-identity', id] as const,
  ghAccounts: ['gh-accounts'] as const,
  credentialDiagnosis: ['credential-diagnosis'] as const,
  tools: ['tools'] as const,
  health: ['health'] as const,
  appIdentity: (path: string) => ['app-identity', path] as const,
};
