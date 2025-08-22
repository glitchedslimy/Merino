<script lang="ts">
    import { closeContextMenu, handleNoteActionContextMenu } from "../../lib/actions/workspace/notes";
    import { currentRightClickedNote } from "../../lib/stores/workspace/notes-store";
    import ContextMenu from "@components/atoms/ContextMenu.svelte";
    import { contextMenuVisible, contextMenuX, contextMenuY } from "../../lib/stores/contextmenu/contextmenu-store";
    import NotesQuickActions from "@components/molecules/NotesQuickActions.svelte";
    import NotesList from "@components/molecules/NotesList.svelte";
        
    const noteContextMenuItems = [
        {
            icon: "trash",
            label: "Delete note",
            action: (name: string) => handleNoteActionContextMenu("delete", name)
        },
        {
            icon: "pencil",
            label: "Rename note",
            action: (name: string) => handleNoteActionContextMenu("rename", name)
        }
    ];
</script>

<section class="flex flex-col gap-y-sm">
    <NotesQuickActions />
    <NotesList />
</section>
{#if $contextMenuVisible && $currentRightClickedNote}
    <ContextMenu
        x={$contextMenuX}
        y={$contextMenuY}
        menuItems={noteContextMenuItems}
        contextId={$currentRightClickedNote.name}
        onclose={closeContextMenu} />
{/if}
