<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import { notifications } from "../../lib/stores/notifications/notification-store";
    import { fade, fly } from "svelte/transition";
    let showNotifications = $state(false)
    let notificationCount: number = $state(0);
    $effect(() => {
        notificationCount = $notifications.length
    })
</script>

<div class="relative px-md pt-xs">
    <Button intent="icon" onClick={() => showNotifications = !showNotifications}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-500" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2V7zm0 8h2v2h-2v-2z"/>
        </svg>
        {#if notificationCount > 0}
            <span transition:fade={{ duration: 200 }} class="absolute top-0 right-2 inline-flex items-center justify-center px-1 py-2 text-xs font-bold leading-none text-red-100 bg-utils-red rounded-full">{notificationCount}</span>
        {/if}
    </Button>
    {#if showNotifications}
        <div class="absolute right-0 mt-2 w-72 bg-black-100 rounded-md shadow-lg py-1 z-50 max-h-80 overflow-y-auto bottom-full"
             transition:fly={{ y: -10, duration: 200 }}>
            {#if notificationCount === 0}
                <div class="text-black-400 text-center py-4">No new notifications.</div>
            {:else}
                {#each $notifications as notification (notification.id)}
                    <div class="p-4 border-b border-gray-200">
                        <p class="text-sm font-semibold">{notification.message}</p>
                        <button onclick={() => notifications.remove(notification.id)} class="text-xs text-brand-primary hover:underline hover:cursor-pointer">Dismiss</button>
                    </div>
                {/each}
            {/if}
        </div>
    {/if}
</div>