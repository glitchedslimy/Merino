<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from 'svelte';
    import { Icon } from "@atoms";

    let isMaximized: boolean = false;

    async function updateMaxState() {
        isMaximized = await getCurrentWindow().isMaximized();
    }

    async function toggleMaximizeMinimize() {
        const appWindow = getCurrentWindow();
        if(isMaximized) {
            await appWindow.unmaximize()
        } else {
            await appWindow.maximize()
        }
        await updateMaxState()
    }

    onMount(() => {
        updateMaxState();

        const unlistenResize = () => getCurrentWindow().onResized(() => {
            updateMaxState();
        })
        return () => {
            unlistenResize();
        }
    })
</script>
<main>
    <div class="w-full flex items-center justify-between bg-black-100 z-20" data-tauri-drag-region>
        <div class="px-3 py-1 pointer-events-none">
            <img src="/src/assets/logo.png" alt="" width="25"/>
        </div>
        <div class="flex">
            <button onclick={() => getCurrentWindow().minimize()} aria-label="minimize button" class="hover:bg-zinc-800 transition duration-200 px-4 py-1">
                <Icon iconName='minimize' width="16"/>
            </button>
            <button onclick={toggleMaximizeMinimize} class="hover:bg-zinc-800 transition duration-200 px-4 py-1" aria-label="maximize/unmaximize button"> 
               {#if isMaximized}
                <Icon iconName='maximizeoff' width="16"/> 

               {:else}
                <Icon iconName='maximize' width="16" /> 
                {/if}
            </button>
            <button class="hover:bg-red-600 transition duration-200 px-4 py-1" aria-label="Close Button" onclick={() => getCurrentWindow().close()}>
               <Icon iconName='close' width="16"/> 
            </button>
        </div>
    </div>
</main>