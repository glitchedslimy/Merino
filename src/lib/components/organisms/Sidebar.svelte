<script>
    import { Icon, Button } from "@atoms";
    import { IconNavigationBar } from "@molecules";
    import { openAiChat } from "@stores/ai-store";
    import { showWorkspace } from "@stores/workspace-store";
    import { fade } from "svelte/transition";
    function openChat() {
        openAiChat.set(!$openAiChat);
    }
</script>

<!-- Sidebar component -->
<aside class="flex flex-col justify-between px-md bg-black-100">
    <IconNavigationBar intent="vertical" verticalSpacing="md" class="py-md">
        <Icon iconName="terminal" width="20" />
        <Icon iconName="search" width="20" />
        {#if !$showWorkspace}
            <div transition:fade={{ duration: 200 }}>
                <Button
                    intent="icon"
                    class="flex space-x-sm"
                    handleClick={openChat}
                >
                    <Icon iconName="WoolyStroke" width="20" />
                </Button>
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
