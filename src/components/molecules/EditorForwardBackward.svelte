<script>
    import NavigationBar from "./NavigationBar.svelte";
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { hideWorkspace } from "../../lib/stores/workspace/workspace-store";
</script>

<NavigationBar intent="horizontal" spacing="h-xs">
    {#if $hideWorkspace}
        <Button intent="icon" onClick={() => $hideWorkspace = !$hideWorkspace}>
            <Icon iconName="chevron-pipe-right" width="20"/>
        </Button>
    {/if}
    <Button intent="icon">
        <Icon iconName="back" width="20"/>
    </Button>
    <Button intent="icon">
        <Icon iconName="forward" width="20"/>
    </Button>
</NavigationBar>
