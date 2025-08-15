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
    activeNoteName, // Changed from activeNoteId
  } from "@stores/workspace-store";
  import { closeNote, openNote } from "@services/internal/editor/notes-buffer";
  import {
    listNotesInSpace,
    deleteNote,
    createNoteInSpace,
    renameNote,
  } from "@services/internal/api/tauri-commands";
  // The Note type is now just a name
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
  let renamingNoteName: string | null = $state(null); // Changed from renamingNoteId
  let inputElement: HTMLInputElement | null = $state(null);
  let timeoutId: number | null = null;
  const DELAY_MS = 500;

  let showTooltip = $state(false);
  let tooltipText = $state("");
  let tooltipX = $state(0);
  let tooltipY = $state(0);

  function handleShowTooltip(text: string, x: number, y: number) {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => {
      showTooltip = true;
      tooltipText = text;
      tooltipX = x;
      tooltipY = y;
    }, DELAY_MS);
  }

  function handleHideTooltip() {
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    showTooltip = false;
    tooltipText = "";
  }
  let { style } = $props();

  const noteContextMenuItems = [
    {
      icon: "trash",
      label: "Delete Note",
      action: (name: string) => handleNoteAction("delete", name),
    },
    {
      icon: "pencil",
      label: "Rename Note",
      action: (name: string) => handleNoteAction("rename", name),
    },
  ];

  async function loadNotesForSpace(spaceName: string | null) {
    console.log("LoadNotes", spaceName);
    if (!spaceName) {
      notes = [];
      loadingNotes = false;
      openNotes.set([]);
      activeNoteName.set(null);
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
      activeNoteName.set(null);
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
          handleNoteAction("delete", currentRightClickedNote.name);
        }
      });
    })();
    return () => {
      if (unlisten) unlisten();
    };
  });

  async function saveRenamedNote(oldName: string, newName: string) {
    if (!newName.trim()) {
      addToast("Note name cannot be empty.", "error");
      renamingNoteName = null;
      return;
    }

    if (oldName && $activeSpaceName) {
      try {
        const renamedNote = await renameNote($activeSpaceName, oldName, newName);

        if (renamedNote) {
          addToast(`Note renamed to "${renamedNote.name}".`, "success");

          // Update the openNotes store
          openNotes.update((notes) => {
            const noteIndex = notes.findIndex((n) => n.name === oldName);
            if (noteIndex !== -1) {
              notes[noteIndex] = renamedNote;
            }
            return notes;
          });

          // Update the active note
          if ($activeNoteName === oldName) {
            openNote(renamedNote);
          }

          await loadNotesForSpace($activeSpaceName);
        } else {
          addToast(`Failed to rename note.`, "error");
        }
      } catch (error) {
        addToast(`Error renaming note: ${error}`, "error");
      }
    }
    renamingNoteName = null;
  }

  async function handleNoteAction(actionType: string, noteName: string) {
    console.log(`[Custom Menu Action] ${actionType} for note name: ${noteName}`);
    switch (actionType) {
      case "delete":
        if (noteName && $activeSpaceName) {
          try {
            const success = await deleteNote($activeSpaceName, noteName);
            if (success) {
              addToast(`Note deleted successfully.`, "success");
              closeNote(noteName);
              await loadNotesForSpace($activeSpaceName);
            } else {
              addToast(`Failed to delete note.`, "error");
              console.warn(`[Action] Failed to delete note name '${noteName}'.`);
            }
          } catch (error) {
            addToast(`Error deleting note: ${error}`, "error");
            console.error("[Action] Error calling delete_note command:", error);
          }
        }
        break;
      case "rename":
        renamingNoteName = noteName;
        await tick();
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
    try {
      // Correctly call the backend command which now generates the name
      const newNote = await createNoteInSpace(spaceName);
      openNote(newNote);
      await loadNotesForSpace(spaceName); // Correctly await the load function
    } catch (e) {
      addToast(`Failed to create note: ${e}`, "error");
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
    $openNotes.find((note) => note.name === $activeNoteName),
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
        {#each notes as note (note.name)}
          <li
            class="list-none"
            oncontextmenu={(e) => showNoteContextMenu(e, note)}
          >
            <Button
              intent="notes"
              handleClick={() => selectNote(note)}
              class={`${activeNote?.name === note.name ? "text-brand-primary-light bg-black-200" : ""} ${currentRightClickedNote?.name === note.name ? "border-[1.5px] border-brand-primary-dark" : ""}`}
            >
              {#if renamingNoteName === note.name}
                <input
                  bind:this={inputElement}
                  type="text"
                  class="w-full h-full p-1 bg-black-300 text-brand-primary-light rounded-sm"
                  value={note.name}
                  onkeydown={(e) => {
                    if (e.key === "Enter") {
                      saveRenamedNote(note.name, e.currentTarget.value);
                    } else if (e.key === "Escape") {
                      renamingNoteName = null;
                    }
                  }}
                  onblur={(e) => saveRenamedNote(note.name, e.currentTarget.value)}
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
    contextId={currentRightClickedNote.name}
    on:close={closeCustomContextMenu}
  />
{/if}