<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import NavigationBar from "./NavigationBar.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { createNote } from "../../lib/actions/workspace/notes";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import { createFolder } from "../../lib/actions/workspace/folders";
</script>
<header>
    <NavigationBar intent="horizontal">
        <Button
            intent="icon"
            class="px-sm"
            onClick={() => createNote($activeSpace || "", null)}
        >
            <Icon iconName="newnote" width="20" />
        </Button>
        <Button intent="icon" class="px-sm" onClick={() => createFolder($activeSpace || "", null)}>
            <Icon iconName="newfolder" width="20" />
        </Button>
    </NavigationBar>
</header>
