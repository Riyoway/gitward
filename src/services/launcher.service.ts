import { openUrl, revealItemInDir } from '@tauri-apps/plugin-opener';

import { invokeResult } from './tauri';
import type { Tool } from '@/types';

/** Detect installed tools, launch them, and reveal/open a repo's locations. */
export const launcherService = {
  detectTools: () => invokeResult<Tool[]>('detect_tools'),
  launchTool: (toolId: string, path: string) =>
    invokeResult<void>('launch_tool', { toolId, path }),
  revealInExplorer: (path: string) => revealItemInDir(path),
  openRemote: (url: string) => openUrl(url),
};
