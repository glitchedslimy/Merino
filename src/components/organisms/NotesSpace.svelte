<script lang="ts">
    import {
        closeContextMenu,
        handleNoteActionContextMenu,
    } from "../../lib/actions/workspace/notes";
    import { currentRightClickedNote } from "../../lib/stores/workspace/notes-store";
    import ContextMenu from "@components/atoms/ContextMenu.svelte";
    import {
        contextMenuVisible,
        contextMenuX,
        contextMenuY,
        folderContextMenuVisible,
        folderContextMenuX,
        folderContextMenuY,
        folderRightClicked,
    } from "../../lib/stores/contextmenu/contextmenu-store";
    import NotesQuickActions from "@components/molecules/NotesQuickActions.svelte";
    import NotesList from "@components/molecules/NotesList.svelte";
    import { closeFolderContextMenu, handleFolderActionContextMenu } from "../../lib/actions/workspace/folders";

    const noteContextMenuItems = [
        {
            icon: "trash",
            label: 'workspace.popUpMenu.folder.delete',
            action: () => handleNoteActionContextMenu("delete"),
        },
        {
            icon: "pencil",
            label: 'workspace.popUpMenu.folder.rename',
            action: () => handleNoteActionContextMenu("rename"),
        },
    ];
    const folderContextMenuItems = [
        {
            icon: "newnote",
            label: 'workspace.popUpMenu.folder.createNote',
            action: () => handleFolderActionContextMenu("create_note"),
        },
        {
            icon: "newfolder",
            label: 'workspace.popUpMenu.folder.create',
            action: () => handleFolderActionContextMenu("create_folder")
        },
        {
            icon: "trash",
            label: 'workspace.popUpMenu.note.delete',
            action: () => handleFolderActionContextMenu("delete")
        },
        {
            icon: "pencil",
            label: 'workspace.popUpMenu.note.rename',
            action: () => handleFolderActionContextMenu("rename")
        }
    ];
</script>

<section class="flex flex-col gap-y-sm relative h-[90%]">
    <NotesQuickActions />
    <NotesList class="flex-1" />
</section>
{#if $folderContextMenuVisible}
    <ContextMenu
        x={$folderContextMenuX}
        y={$folderContextMenuY}
        menuItems={folderContextMenuItems}
        contextId={$folderRightClicked?.name}
        onclose={closeFolderContextMenu}
    />
{/if}
{#if $contextMenuVisible && $currentRightClickedNote}
    <ContextMenu
        x={$contextMenuX}
        y={$contextMenuY}
        menuItems={noteContextMenuItems}
        contextId={$currentRightClickedNote.name}
        onclose={closeContextMenu}
    />
{/if}