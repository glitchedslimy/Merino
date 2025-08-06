<script lang="ts">
    import { toasts } from '@stores/toast-store';
    import Toast from '@components/atoms/Toast.svelte'; // Adjust path as needed
</script>

<div class="fixed bottom-0 right-sm z-[1000] pointer-events-none">
    {#each $toasts as toast (toast.id)}
        <div class="pointer-events-auto">
            <Toast {...toast} />
        </div>
    {/each}
</div>