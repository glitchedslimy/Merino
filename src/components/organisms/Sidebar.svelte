<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import NavigationBar from "@components/molecules/NavigationBar.svelte";
    import { showCommandPalette } from "../../lib/stores/commandpalette/commandpalette";
    import { hideWorkspace } from "../../lib/stores/workspace/workspace-store";
    import { fade } from "svelte/transition";
    import { openAiChat } from "../../lib/stores/ai/ai-store";
    import { openSettings } from "../../lib/stores/settings/settings";
</script>

<div class="h-full flex justify-between flex-col items-center px-md">
    <NavigationBar
        intent="vertical"
        spacing="v-md"
        justify="center"
        class="pt-md"
    >
        <Button
            intent="icon"
            onClick={() => ($showCommandPalette = !$showCommandPalette)}
        >
            <Icon iconName="terminal" width="20" />
        </Button>
        <Button
            intent="icon"
            onClick={() => ($showCommandPalette = !$showCommandPalette)}
        >
            <Icon iconName="search" width="20" />
        </Button>
        {#if $hideWorkspace}
            <div transition:fade={{ duration: 200 }} class="flex flex-col gap-y-md">
                <Button
                    intent="icon"
                    onClick={() => ($openAiChat = !$openAiChat)}
                >
                    <Icon iconName="WoolyStroke" width="20" />
                </Button>
                <Button
                    intent="icon"
                    onClick={() => ($showCommandPalette = !$showCommandPalette)}
                >
                    <Icon iconName="templates" width="20" />
                </Button>
            </div>
        {/if}
    </NavigationBar>
    <Button intent="icon" class="pb-md" onClick={() => $openSettings = !$openSettings}>
        <Icon iconName="settings" width="20" />
    </Button>
</div>
