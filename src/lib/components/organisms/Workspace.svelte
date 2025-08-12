<script lang="ts">
  import { slide } from "svelte/transition";
  import { get } from "svelte/store";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { addToast } from "@stores/toast-store";
  import { openAiChat } from "@stores/ai-store";
  import {
    showWorkspace,
    activeSpaceName,
    loadPersistentState,
    openNotes,
    activeNoteId,
  } from "@stores/workspace-store";
  import { closeNote, openNote } from "@services/internal/editor/notes-buffer";
  import {
    listNotesInSpace,
    deleteNote,
    createNoteInSpace,
    renameNote,
  } from "@services/internal/api/tauri-commands";
  import type { Note } from "@services/internal/api/models/rust-models";

  import { Button, Icon, SpaceDropdownTest } from "@atoms";
  import IconNavigationBar from "@components/molecules/IconNavigationBar.svelte";
  import ContextualMenu from "@components/atoms/ContextualMenu.svelte";
  import { handleCloseMySpace } from "@services/internal/workspace/control-hide-workspace";
  import { tick } from "svelte";
  import Tooltip from "@components/atoms/Tooltip.svelte";
  import { ellipsisTooltip } from "../../utils/ellipsis-detection";

  let notes: Note[] = $state([]);
  let loadingNotes = $state(false);
  let customMenuVisible = $state(false);
  let customMenuX = $state(0);
  let customMenuY = $state(0);
  let currentRightClickedNote: Note | null = $state(null);
  let renamingNoteId: string | null = $state(null); // New state variable
  let inputElement: HTMLInputElement | null = null;
  let timeoutId: number | null = null;
  const DELAY_MS = 500;

  let showTooltip = $state(false);
  let tooltipText = $state("");
  let tooltipX = $state(0);
  let tooltipY = $state(0);

  function handleShowTooltip(text: string, x: number, y: number) {
    // Clear any existing timeout to restart the timer
    if (timeoutId) {
      clearTimeout(timeoutId);
    }

    // Start a new timer
    timeoutId = setTimeout(() => {
      showTooltip = true;
      tooltipText = text;
      tooltipX = x;
      tooltipY = y;
    }, DELAY_MS);
  }

  function handleHideTooltip() {
    // Clear the timeout to prevent the tooltip from appearing
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    // Hide the tooltip immediately
    showTooltip = false;
    tooltipText = "";
  }
  let { style } = $props();

  const noteContextMenuItems = [
    {
      icon: "trash",
      label: "Delete Note",
      action: (id: string) => handleNoteAction("delete", id),
    },
    {
      icon: "pencil",
      label: "Rename Note",
      action: (id: string) => handleNoteAction("rename", id),
    },
  ];

  async function loadNotesForSpace(spaceName: string | null) {
    console.log("LoadNotes", spaceName);
    if (!spaceName) {
      notes = [];
      loadingNotes = false;
      openNotes.set([]);
      activeNoteId.set(null);
      return;
    }
    loadingNotes = true;
    try {
      const loadedNotes = await listNotesInSpace(spaceName);
      console.log("Loaded notes", loadedNotes);
      notes = loadedNotes;
      if (loadedNotes.length > 0 && get(openNotes).length === 0) {
        openNote(loadedNotes[0]);
      }
    } catch (e) {
      console.error("Failed to load notes:", e);
      notes = [];
      openNotes.set([]);
      activeNoteId.set(null);
      addToast(`Error loading notes: ${e}`, "error");
    } finally {
      loadingNotes = false;
    }
  }

  $effect(() => {
    loadNotesForSpace($activeSpaceName);
  });

  $effect.root(() => {
    loadPersistentState();
    let unlisten: UnlistenFn | undefined;
    (async () => {
      unlisten = await listen("tauri://menu", (event) => {
        if (event.payload === "delete-note" && currentRightClickedNote) {
          handleNoteAction("delete", currentRightClickedNote.id);
        }
      });
    })();
    return () => {
      if (unlisten) unlisten();
    };
  });

  async function saveRenamedNote(id: string, newName: string) {
    // Basic validation
    if (!newName.trim()) {
      addToast("Note name cannot be empty.", "error");
      renamingNoteId = null;
      return;
    }

    if (id && $activeSpaceName) {
      try {
        // Call the Tauri command to rename the note on the backend
        const renamedNote = await renameNote($activeSpaceName, id, newName);

        if (renamedNote) {
          addToast(`Note renamed to "${renamedNote.name}".`, "success");

          // Update the openNotes store with the new name
          openNotes.update((notes) => {
            const noteIndex = notes.findIndex((n) => n.id === id);
            if (noteIndex !== -1) {
              notes[noteIndex] = renamedNote;
            }
            return notes;
          });

          // Update the active note if it's the one being renamed
          if ($activeNoteId === id) {
            openNote(renamedNote);
          }

          // Reload the notes for the sidebar to ensure consistency
          await loadNotesForSpace($activeSpaceName);
        } else {
          addToast(`Failed to rename note.`, "error");
        }
      } catch (error) {
        addToast(`Error renaming note: ${error}`, "error");
      }
    }
    renamingNoteId = null; // Reset the state to show the note name again
  }

  async function handleNoteAction(actionType: string, noteId: string) {
    console.log(`[Custom Menu Action] ${actionType} for note ID: ${noteId}`);
    switch (actionType) {
      case "delete":
        if (noteId && $activeSpaceName) {
          try {
            const success = await deleteNote($activeSpaceName, noteId);
            if (success) {
              addToast(`Note deleted successfully.`, "success");
              closeNote(noteId);
              await loadNotesForSpace($activeSpaceName);
            } else {
              addToast(`Failed to delete note.`, "error");
              console.warn(`[Action] Failed to delete note ID '${noteId}'.`);
            }
          } catch (error) {
            addToast(`Error deleting note: ${error}`, "error");
            console.error("[Action] Error calling delete_note command:", error);
          }
        }
        break;
      case "rename":
        renamingNoteId = noteId; // Set the ID to trigger the input field
        await tick(); // Wait for the DOM to update
        if (inputElement) {
          inputElement.focus();
        }
        break;
      default:
        console.log(`[Action] Unimplemented custom menu action: ${actionType}`);
    }
    customMenuVisible = false;
  }

  function selectNote(note: Note) {
    openNote(note);
  }

  async function createNote(spaceName: string) {
    const finalNoteName = "Sin titulo";
    try {
      const newNote = await createNoteInSpace(spaceName, {
        name: finalNoteName,
        content: "",
      });
      openNote(newNote);
      let notes = await loadNotesForSpace(spaceName);
      console.log(notes);
    } catch (e) {
      addToast(e, "error");
      console.error("Failed to create note:", e);
    }
  }

  function showNoteContextMenu(event: MouseEvent, note: Note) {
    event.preventDefault();
    customMenuX = event.clientX;
    customMenuY = event.clientY;
    currentRightClickedNote = note;
    customMenuVisible = true;
    console.log(`[ContextMenu] Right-clicked note:`, note.name);
  }

  function closeCustomContextMenu() {
    customMenuVisible = false;
    currentRightClickedNote = null;
  }

  function openChat() {
    openAiChat.set(!$openAiChat);
  }

  let activeNote = $derived(
    $openNotes.find((note) => note.id === $activeNoteId),
  );
</script>

{#if $showWorkspace}
  <div
    class="flex flex-col space-y-md px-lg py-md bg-black rounded-md w-72 overflow-hidden"
    transition:slide={{ duration: 200, axis: "x" }}
    {style}
  >
    <div class="flex items-center justify-between">
      <div class="flex space-x-xs items-center">
        <Icon iconName="home" />
        <SpaceDropdownTest />
      </div>
      <span>
        <Button intent="icon" handleClick={handleCloseMySpace}>
          <Icon iconName="chevron-pipe-left" width="20" />
        </Button>
      </span>
    </div>
    <div>
      <ul>
        <li class="text-black-400 flex space-x-xs items-center">
          <Button intent="icon" class="flex space-x-sm" handleClick={openChat}>
            <Icon iconName="WoolyStroke" width="20" />
            <span>Ask Wooly</span>
          </Button>
        </li>
        <li class="text-black-400 flex space-x-xs items-center">
          <Icon iconName="templates" width="20" />
          <p>Templates (disabled)</p>
        </li>
      </ul>
    </div>
    <div class="flex-1 flex flex-col mt-md">
      <IconNavigationBar intent="horizontal" horizontalSpacing="md">
        <Button
          intent="icon"
          handleClick={() => createNote($activeSpaceName || "")}
        >
          <Icon iconName="newnote" width="20" />
        </Button>
        <Button intent="icon">
          <Icon iconName="newfolder" width="20" />
        </Button>
      </IconNavigationBar>
      <div class="mt-md pr-4 overflow-y-auto h-[75%] no-scroll">
        {#each notes as note (note.id)}
          <li
            class="list-none"
            oncontextmenu={(e) => showNoteContextMenu(e, note)}
          >
            <Button
              intent="notes"
              handleClick={() => selectNote(note)}
              class={`${activeNote?.id === note.id ? "text-brand-primary-light bg-black-200" : ""} ${currentRightClickedNote?.id === note.id ? "border-[1.5px] border-brand-primary-dark" : ""}`}
            >
              {#if renamingNoteId === note.id}
                <input
                  bind:this={inputElement}
                  type="text"
                  class="w-full h-full p-1 bg-black-300 text-brand-primary-light rounded-sm"
                  value={note.name.replace(/\.[^/.]+$/, "")}
                  onkeydown={(e) => {
                    if (e.key === "Enter") {
                      saveRenamedNote(note.id, e.currentTarget.value);
                    } else if (e.key === "Escape") {
                      renamingNoteId = null;
                    }
                  }}
                  onblur={(e) =>
                    saveRenamedNote(note.id, e.currentTarget.value)}
                />
              {:else}
                <Icon iconName="note" width="20" />
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
        {:else}
          <li class="text-black-300 list-none">
            No hay notas en este espacio.
          </li>
        {/each}
      </div>
    </div>
  </div>
{/if}

{#if showTooltip}
  <Tooltip text={tooltipText} x={tooltipX} y={tooltipY} />
{/if}
{#if customMenuVisible && currentRightClickedNote}
  <ContextualMenu
    x={customMenuX}
    y={customMenuY}
    menuItems={noteContextMenuItems}
    contextId={currentRightClickedNote.id}
    on:close={closeCustomContextMenu}
  />
{/if}
