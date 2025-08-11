<script lang="ts">
  import { Button, Icon, SpaceDropdownTest } from "@atoms";
  import IconNavigationBar from "@components/molecules/IconNavigationBar.svelte";
  import { handleCloseMySpace } from "@services/internal/workspace/control-hide-workspace";
  import {
    showWorkspace,
    activeSpaceName,
    loadPersistentState,
    activeNote,
  } from "@stores/workspace-store";
  import { slide } from "svelte/transition";
  import {
    createNoteInSpace,
    listNotesInSpace,
    deleteNote,
  } from "@services/internal/api/tauri-commands";
  import type { Note } from "@services/internal/api/models/rust-models";
  import { get } from "svelte/store";
  import { addToast } from "@stores/toast-store";
  import { Menu } from "@tauri-apps/api/menu";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import ContextualMenu from "@components/atoms/ContextualMenu.svelte";
    import { openAiChat } from "@stores/ai-store";

  let notes: Note[] = $state([]);
  let loadingNotes = $state(false);
  let customMenuVisible = $state(false);
  let customMenuX = $state(0);
  let customMenuY = $state(0);
  let currentRightClickedNote: Note | null = $state(null);
  
  const noteContextMenuItems = [
    {
      label: "Edit Note",
      action: (id: string) => handleNoteAction("edit", id),
    },
    {
      label: "Delete Note",
      action: (id: string) => handleNoteAction("delete", id),
    },
    {
      label: "Share Note",
      action: (id: string) => handleNoteAction("share", id),
    },
  ];

  async function loadNotesForSpace(spaceName: string) {
    if (!spaceName) {
      notes = [];
      loadingNotes = false;
      activeNote.set(null);
      return;
    }
    loadingNotes = true;
    try {
      const loadedNotes = await listNotesInSpace(spaceName);
      notes = loadedNotes;
      if (loadedNotes.length > 0 && !get(activeNote)) {
        activeNote.set(loadedNotes[0]);
      }
    } catch (e) {
      console.error("Failed to load notes:", e);
      notes = [];
      activeNote.set(null);
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
      unlisten = await listen('tauri://menu', (event) => {
        if (event.payload === 'delete-note' && currentRightClickedNote) {
          handleNoteAction("delete", currentRightClickedNote.name);
        }
      });
    })();
    return () => {
      if (unlisten) unlisten();
    };
  });

  async function handleNoteAction(actionType: string, noteName: string) {
    console.log(`[Custom Menu Action] ${actionType} for note: ${noteName}`);
    switch (actionType) {
      case "edit":
        console.log(`[Action] Editing note: ${noteName}`);
        break;
      case "delete":
        if (noteName && $activeSpaceName) {
          try {
            const success = await deleteNote($activeSpaceName, noteName);
            if (success) {
              addToast(`Note '${noteName}' deleted successfully.`, "success");
              await loadNotesForSpace($activeSpaceName);
              if (get(activeNote)?.name === noteName) {
                activeNote.set(null);
              }
            } else {
              addToast(`Failed to delete note '${noteName}'.`, "error");
              console.warn(`[Action] Failed to delete note '${noteName}'.`);
            }
          } catch (error) {
            addToast(`Error deleting note: ${error}`, "error");
            console.error("[Action] Error calling delete_note command:", error);
          }
        } else {
          console.warn("[Action] No note name or active space for deletion.");
        }
        break;
      case "share":
        console.log(`[Action] Sharing note: ${noteName}`);
        break;
      default:
        console.log(`[Action] Unimplemented custom menu action: ${actionType}`);
    }
    customMenuVisible = false;
  }

  function selectNote(note: Note) {
    console.log("Selecting note:", note);
    activeNote.set(note);
  }

  async function createNote(spaceName: string) {
    let newNoteTitle = "Sin titulo";
    let counter = 1;
    let finalNoteName = `${newNoteTitle} ${counter}`;

    while (notes.some((note) => note.name === finalNoteName)) {
      counter++;
      finalNoteName = `${newNoteTitle} ${counter}`;
    }

    try {
      const newNote = await createNoteInSpace(spaceName, {
        name: finalNoteName,
        content: "",
      });
      activeNote.set(newNote);
      await loadNotesForSpace(spaceName);
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
</script>

{#if $showWorkspace}
  <div
    class="flex flex-col space-y-md px-lg py-md bg-black rounded-md w-72 overflow-hidden"
    transition:slide={{ duration: 200, axis: "x" }}
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
        <Button intent="icon" handleClick={() => createNote($activeSpaceName)}>
          <Icon iconName="newnote" width="20" />
        </Button>
        <Button intent="icon">
          <Icon iconName="newfolder" width="20" />
        </Button>
      </IconNavigationBar>
      <div class="mt-md pr-4 overflow-y-auto h-[75%] no-scroll">
        {#each notes as note}
          <li
            class="list-none"
            oncontextmenu={(e) => showNoteContextMenu(e, note)}
          >
            <Button
              intent="notes"
              handleClick={() => selectNote(note)}
              class={`${$activeNote?.name === note.name ? "text-brand-primary-light bg-black-200" : ""} ${currentRightClickedNote?.name === note.name ? "border-[1.5px] border-brand-primary-dark" : ''}`}
            >
              <Icon iconName="note" width="20" />
              <span>{note.name}</span>
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

{#if customMenuVisible}
  <ContextualMenu
    x={customMenuX}
    y={customMenuY}
    menuItems={noteContextMenuItems}
    contextId={currentRightClickedNote?.name || ""}
    on:close={closeCustomContextMenu}
  />
{/if}