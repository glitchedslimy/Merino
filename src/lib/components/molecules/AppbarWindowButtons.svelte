<script lang="ts">
    import { Button, Icon } from "@atoms";
    /**
     * Obtain the current window object and its methods
     * as well, import the onMount to manage window state.
    */
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";

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
    });
</script>

<div class="flex  justify-end">
        <Button intent='appbar' handleClick={() => getCurrentWindow().minimize()}>
            <Icon iconName='minimize' width=16 />
        </Button>
        <Button intent="appbar" handleClick={toggleMaximizeMinimize}>
            {#if isMaximized}
                <Icon iconName='maximizeoff' width='16' />
            {:else}
                <Icon iconName="maximize" width='16'/> 
            {/if}
        </Button>
        <Button intent="appbarclose" handleClick={() => getCurrentWindow().close()}>
            <Icon iconName="close" width="16" />
       </Button> 
    </div>