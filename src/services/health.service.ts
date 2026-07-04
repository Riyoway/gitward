import { invokeResult } from './tauri';
import type { HealthReport } from '@/types';

/** Check whether git/gh/ssh/internet and known tools are available. */
export const healthService = {
  check: () => invokeResult<HealthReport>('health_check'),
};
