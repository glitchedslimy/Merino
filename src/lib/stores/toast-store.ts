import { writable } from 'svelte/store';

// Define the shape of a single toast message
export interface ToastMessage {
    id: string;
    message: string;
    type: 'success' | 'error' | 'info' | 'warning';
    duration?: number; // Optional duration in milliseconds
}

// Create a writable store to hold an array of toast messages
export const toasts = writable<ToastMessage[]>([]);

// Function to add a toast message
export function addToast(message: string, type: ToastMessage['type'] = 'info', duration: number = 3000) {
    const id = Date.now().toString(); // Simple unique ID
    toasts.update(currentToasts => [
        ...currentToasts,
        { id, message, type, duration }
    ]);

    // Automatically remove the toast after its duration
    setTimeout(() => {
        removeToast(id);
    }, duration);
}

// Function to remove a toast message by its ID
export function removeToast(id: string) {
    toasts.update(currentToasts => currentToasts.filter(toast => toast.id !== id));
}