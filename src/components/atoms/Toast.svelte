<script lang="ts">
    import { onMount } from 'svelte';
    import { toasts } from '../../lib/stores/notifications/toast-store';
    import Button from './Button.svelte';
    import Icon from './Icon.svelte';

    export let message: string;
    export let type: 'success' | 'error' | 'info';
    export let id: number;

    // Automatically remove the toast after a few seconds
    onMount(() => {
        const timeout = setTimeout(() => {
            toasts.remove(id);
        }, 3000); // 3 seconds
        return () => clearTimeout(timeout);
    });

    // Determine colors based on type
    const colorClasses = {
        success: 'bg-utils-green-dark text-white',
        error: 'bg-utils-red-dark text-white',
        info: 'bg-utils-blue-dark text-white',
    };
</script>

<div class="p-4 rounded-md shadow-md {colorClasses[type]} mb-sm">
    <div class="flex items-center justify-between gap-x-sm">
        <p class="font-medium">{message}</p>
        <Button intent="icon" onClick={() => toasts.remove(id)}>
            <Icon iconName="close" width="20"/>
        </Button>
    </div>
</div>