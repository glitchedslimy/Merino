<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import {
        handleHideTooltip,
        handleNoteActionContextMenu,
        handleShowTooltip,
        loadNotesInSpace,
        renameNote,
        selectNote,
        showNoteContextmenu,
    } from "../../lib/actions/workspace/notes";
    import {
        activeNoteName,
        currentRightClickedNote,
        inputRenameNoteElement,
        notes,
        opennedNotes,
        renamedNoteName,
    } from "../../lib/stores/workspace/notes-store";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import Input from "@components/atoms/Input.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { ellipsisTooltip } from "../../lib/useHooks/ellipsisTooltip";
    import {
        showTooltip,
        tooltipText,
        tooltipX,
        tooltipY,
    } from "../../lib/stores/tooltip/tooltip-store";
    import Tooltip from "@components/atoms/Tooltip.svelte";
    import { contextMenuVisible } from "../../lib/stores/contextmenu/contextmenu-store";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { loadFoldersInSpace } from "../../lib/actions/workspace/folders";
    import { folders } from "../../lib/stores/workspace/folders-store";

    $effect(() => {
        loadFoldersInSpace($activeSpace)
        loadNotesInSpace($activeSpace);
        let unlisten: UnlistenFn | undefined;
        (async () => {
            unlisten = await listen("tauri://menu", (event) => {
                if (
                    event.payload === "delete-note" &&
                    $currentRightClickedNote
                ) {
                    handleNoteActionContextMenu(
                        "delete",
                        $currentRightClickedNote.name,
                    );
                }
            });
        })();
        return () => {
            if (unlisten) unlisten();
        };
    });

    let activeNote = $derived(
        $opennedNotes.find((note) => note.name === $activeNoteName),
    );

    function handleDragStart(e: DragEvent, note: string) {
        console.log("HandleDragstart")
        e.dataTransfer?.setData('text/plain', note)
    }

    function handleDragOver(event: DragEvent) {
        console.log("Dragover folder")
        event.preventDefault();
    }

    async function handleDrop(event: DragEvent, folder: string) {
        console.log("DROP in folder")
        event.preventDefault();
        // Get the note's name from the drag event's dataTransfer
        const draggedNoteName = event.dataTransfer?.getData('text/plain');

        if (draggedNoteName && folder) {
            console.log(`Attempting to move note '${draggedNoteName}' into folder '${folder}'`);
            
            // Call the action to move the note on the backend
            // await moveNoteToFolder(draggedNoteName, folder);
        }
    }
</script>

<section class="px-xs">
    {#each $folders as folderName (folderName)}
        <ul>
            <li>
                <Button intent="notes" class={`flex items-center gap-x-xs`} ondragover={handleDragOver} ondrop={(e: any) => handleDrop(e, folderName.name)}>
                    <Icon iconName="folder" />
                    {folderName.name}
                </Button>
            </li>
        </ul>
    {/each}
    {#each $notes as note (note.name)}
        <ul>
            <li oncontextmenu={(e) => showNoteContextmenu(e, note)}>
                <Button
                    intent="notes"
                    onClick={() => selectNote(note)}
                    class={`${activeNote?.name === note.name ? "text-brand-primary-light bg-black-200" : ""} ${$currentRightClickedNote?.name === note.name ? "border-[1.5px] border-brand-primary-dark" : ""} flex items-center gap-x-xs`}
                    draggable={true}
                    ondragstart={(e: any) => handleDragStart(e, note.name)}
                >
                    <Icon iconName="note" width="20" />
                    {#if $renamedNoteName === note.name}
                        <Input
                            bind:inputElement={$inputRenameNoteElement}
                            inputType="text"
                            value={note.name}
                            class=""
                            onKeydown={(e: any) => {
                                if (e.key === "Enter") {
                                    console.log("Saved note");
                                    renameNote(
                                        note.name,
                                        e.currentTarget?.value,
                                    );
                                } else if (e.key === "Escape") {
                                    renamedNoteName.set(null);
                                }
                            }}
                            onblur={(e: any) => {
                                renameNote(note.name, e.currentTarget?.value);
                            }}
                        />
                    {:else}
                        <span
                            use:ellipsisTooltip={{
                                onShow: handleShowTooltip,
                                onHide: handleHideTooltip,
                            }}
                            class="text-ellipsis whitespace-nowrap overflow-hidden"
                            >{note.name}</span
                        >
                    {/if}
                </Button>
            </li>
        </ul>
    {:else}
        <p class="text-black-300">No notes in this space.</p>
    {/each}
</section>
{#if $showTooltip && !$contextMenuVisible}
    <Tooltip text={$tooltipText} x={$tooltipX} y={$tooltipY} />
{/if}
