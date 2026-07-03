/** Derive a display name from a path: its last path segment (POSIX or Windows). */
export function deriveRepoName(path: string): string {
  const segments = path.split(/[\\/]+/).filter(Boolean);
  return segments.at(-1) ?? path;
}
