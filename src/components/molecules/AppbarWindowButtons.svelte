<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    /**
     * Obtain the current window object and its methods
     * as well, import the onMount to manage window state.
     */
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import NavigationBar from "./NavigationBar.svelte";
    import { toggleMaximizeMinimize, updateMaxState } from "../../lib/actions/window/maximize";
    import { isMaximized } from "../../lib/stores/window/window";

    onMount(() => {
        updateMaxState();

        const unlistenResize = () =>
            getCurrentWindow().onResized(() => {
                updateMaxState();
            });
        return () => {
            unlistenResize();
        };
    });
</script>

<NavigationBar intent="horizontal">
    <Button intent="window" onClick={() => getCurrentWindow().minimize()}>
        <Icon iconName="minimize" width="16" />
    </Button>
    <Button intent="window" onClick={toggleMaximizeMinimize}>
        {#if $isMaximized}
            <Icon iconName="maximizeoff" width="16" />
        {:else}
            <Icon iconName="maximize" width="16" />
        {/if}
    </Button>
    <Button intent="windowClose" onClick={() => getCurrentWindow().close()}>
        <Icon iconName="close" width="16" />
    </Button>
</NavigationBar>
