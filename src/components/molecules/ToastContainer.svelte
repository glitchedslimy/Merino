<script lang="ts">
    import Toast from '@components/atoms/Toast.svelte'; // Adjust path
    import { fade } from 'svelte/transition';
    import { toasts } from '../../lib/stores/notifications/toast-store';
</script>

<div class="fixed bottom-0 right-0 z-[1000] flex flex-col items-end pointer-events-none p-4">
    {#each $toasts as toast (toast.id)}
        <div class="pointer-events-auto" transition:fade>
            <Toast {...toast} />
        </div>
    {/each}
</div>