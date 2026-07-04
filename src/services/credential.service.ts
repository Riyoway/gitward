import { invokeResult } from './tauri';
import type { CredentialDiagnosis } from '@/types';

/** Diagnose gh vs Git Credential Manager alignment. */
export const credentialService = {
  diagnose: () => invokeResult<CredentialDiagnosis>('credential_diagnose'),
};
