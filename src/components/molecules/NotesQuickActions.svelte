<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import NavigationBar from "./NavigationBar.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { createNote } from "../../lib/actions/workspace/notes";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
</script>
<header>
    <NavigationBar intent="horizontal">
        <Button
            intent="icon"
            class="px-sm"
            onClick={() => createNote($activeSpace || "")}
        >
            <Icon iconName="newnote" width="20" />
        </Button>
        <Button intent="icon" class="px-sm">
            <Icon iconName="newfolder" width="20" />
        </Button>
    </NavigationBar>
</header>
