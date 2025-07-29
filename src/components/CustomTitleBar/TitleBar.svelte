<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from 'svelte';

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
        <div class="px-4 py-1 pointer-events-none">
            <img src="/src/assets/logo.png" alt="" width="25"/>
        </div>
        <div class="flex">
            <button onclick={() => getCurrentWindow().minimize()} aria-label="minimize button" class="hover:bg-zinc-800 transition duration-200 px-3 py-1">
                <svg  xmlns="http://www.w3.org/2000/svg"  width="16"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-minus"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M5 12l14 0" /></svg>
            </button>
            <button onclick={toggleMaximizeMinimize} class="hover:bg-zinc-800 transition duration-200 px-3 py-1" aria-label="maximize/unmaximize button"> 
               {#if isMaximized}
               <svg  xmlns="http://www.w3.org/2000/svg"  width="16"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-squares"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M8 10a2 2 0 0 1 2 -2h9a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-9a2 2 0 0 1 -2 -2z" /><path d="M16 8v-3a2 2 0 0 0 -2 -2h-9a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h3" /></svg>
               {:else}
                <svg  xmlns="http://www.w3.org/2000/svg"  width="16"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-square"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" /></svg>
                {/if}
            </button>
            <button class="hover:bg-red-600 transition duration-200 px-3 py-1" aria-label="Close Button" onclick={() => getCurrentWindow().close()}>
                <svg  xmlns="http://www.w3.org/2000/svg"  width="16"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-x"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M18 6l-12 12" /><path d="M6 6l12 12" /></svg>
            </button>
        </div>
    </div>
</main>