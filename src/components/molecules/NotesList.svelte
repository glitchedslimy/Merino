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
    import NotePreview from "@components/atoms/NotePreview.svelte";
    import { getNoteContent } from "../../lib/api/tauri/get/notes-api-get";

    $effect(() => {
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
</script>

<section class="px-xs">
    {#each $notes as note (note.name)}
        <ul>
            <li oncontextmenu={(e) => showNoteContextmenu(e, note)}>
                <Button
                    intent="notes"
                    onClick={() => selectNote(note)}
                    class={`${activeNote?.name === note.name ? "text-brand-primary-light bg-black-200" : ""} ${$currentRightClickedNote?.name === note.name ? "border-[1.5px] border-brand-primary-dark" : ""} flex items-center gap-x-xs`}
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
