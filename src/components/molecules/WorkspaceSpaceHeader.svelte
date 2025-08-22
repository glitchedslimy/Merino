 <script lang="ts">
    import { loadSpaces } from "../../lib/actions/workspace/spaces";
    import type { Space } from "../../lib/api/tauri/interfaces/spaces-interface";
    import { loadPersistentSpaceNameState, saveActiveNameSpace } from "../../lib/stores/workspace/spaces-store";
    import { spacesStore } from "../../lib/stores/workspace/spaces-store";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import Button from "@components/atoms/Button.svelte";
    import { hideWorkspace } from "../../lib/stores/workspace/workspace-store";
    import { activeNoteName, opennedNotes } from "../../lib/stores/workspace/notes-store";
    
    let loadedSpaces: Space[] | undefined = $derived($spacesStore.spaces)
    let selectedOption = $derived($activeSpace || "My Space")
    
    function selectSpace(spaceName: string | null) {
        activeSpace.set(spaceName)
        saveActiveNameSpace(spaceName)
        opennedNotes.set([])
        activeNoteName.set(null)
    } 
    
    $effect(() => {
        loadPersistentSpaceNameState()
        loadSpaces();
    })
 </script>
 
 <header class="flex justify-between gap-x-md">
        <Dropdown options={loadedSpaces} onSelect={selectSpace} selectedOption={selectedOption} icon="spaces"/>
        <Button intent="icon" onClick={() => $hideWorkspace = !$hideWorkspace}>
            <Icon iconName="chevron-pipe-left" width="20"/>
        </Button>
</header>