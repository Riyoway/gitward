import { addToast } from '@heroui/react';

export function toastSuccess(title: string, description?: string) {
  addToast({ title, description, color: 'success' });
}

export function toastError(title: string, description?: string) {
  addToast({ title, description, color: 'danger' });
}
