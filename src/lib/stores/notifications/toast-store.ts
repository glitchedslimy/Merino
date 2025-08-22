import { writable } from 'svelte/store';

export type Toast = {
    id: number;
    message: string;
    type: 'success' | 'error' | 'info';
};

function createToastStore() {
    const { subscribe, update } = writable<Toast[]>([]);

    return {
        subscribe,
        add: (message: string, type: 'success' | 'error' | 'info') => {
            const newToast = {
                id: Date.now(),
                message,
                type
            };
            update(toasts => [...toasts, newToast]);
        },
        remove: (id: number) => {
            update(toasts => toasts.filter(t => t.id !== id));
        },
    };
}

export const toasts = createToastStore();