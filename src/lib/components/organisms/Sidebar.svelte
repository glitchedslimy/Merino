<script>
    import { Icon, Button } from "@atoms";
    import { IconNavigationBar } from "@molecules";
    import { showWorkspace } from "@stores/workspace-store";
    import { fade } from "svelte/transition";
</script>

<!-- Sidebar component -->
<aside class="flex flex-col justify-between px-md bg-black-100">
    <IconNavigationBar intent="vertical" verticalSpacing="md" class="py-md">
        <Icon iconName="terminal" width="20" />
        <Icon iconName="search" width="20" />
        {#if !$showWorkspace}
            <div transition:fade={{ duration: 200 }}>
                <Icon iconName="ai" />
            </div>
            <div transition:fade={{ duration: 200 }}>
                <Icon iconName="templates" />
            </div>
        {/if}
    </IconNavigationBar>
    <div class="py-xs">
        <Button intent="icon">
            <Icon iconName="settings" width="20" />
        </Button>
    </div>
</aside>
