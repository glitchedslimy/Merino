<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { removeToast } from '@stores/toast-store';
    import Icon from './Icon.svelte'; // Assuming you have an Icon component
    import { Button } from '@atoms';

    // Props for the individual toast
    let { id, message, type } = $props();

    // Dynamic classes based on toast type
    let bgColor: string;
    let textColor: string;
    let iconName: string;

    $effect(() => {
        switch (type) {
            case 'success':
                bgColor = 'bg-green-500';
                textColor = 'text-white';
                iconName = 'check-circle'; // Example icon name
                break;
            case 'error':
                bgColor = 'bg-red-500';
                textColor = 'text-white';
                iconName = 'x-circle'; // Example icon name
                break;
            case 'info':
                bgColor = 'bg-blue-500';
                textColor = 'text-white';
                iconName = 'info-circle'; // Example icon name
                break;
            case 'warning':
                bgColor = 'bg-yellow-500';
                textColor = 'text-black';
                iconName = 'exclamation-triangle'; // Example icon name
                break;
            default:
                bgColor = 'bg-gray-700';
                textColor = 'text-white';
                iconName = 'bell'; // Default icon
        }
    });

    function handleClose() {
        removeToast(id);
    }
</script>

<div
    class="flex items-center justify-between rounded-md shadow-lg px-2 mb-4 py-2 w-full space-x-sm"
    class:bg-utils-green-dark={type === 'success'}
    class:bg-utils-red-dark={type === 'error'}
    class:bg-utils-blue-dark={type === 'info'}
    class:bg-utils-yellow-bright={type === 'warning'}
    class:text-white={type !== 'warning'}
    class:text-black={type === 'warning'}
    transition:fly={{ y: 50, duration: 300 }}
>
    <div class="flex items-center">
        {#if iconName}
            <!-- Assuming your Icon component takes an iconName prop -->
            <Icon iconName={iconName} />
        {/if}
        <p class="">{message}</p>
    </div>
    <Button
        handleClick={handleClose}
        intent="icon"
    >
        <Icon iconName="close" />
    </Button>
</div>

