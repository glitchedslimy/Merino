import { writable } from "svelte/store";

export type Notification = {
    id: number;
    message: string;
    type: 'sucess' | 'error' | 'info' 
}

function createNotificationStore() {
    const { subscribe, update } = writable<Notification[]>([]);

    return {
        subscribe,
        add: (message: string, type: 'success' | 'error' | 'info') => {
            const newNotification = {
                id: Date.now(), // Unique ID
                message,
                type
            };
            update((notifications: any) => [...notifications, newNotification]);
        },
        remove: (id: number) => {
            update((notifications: any) => notifications.filter((n: any) => n.id !== id));
        },
        clear: () => {
            update(() => []);
        }
    };
}

export const notifications = createNotificationStore();