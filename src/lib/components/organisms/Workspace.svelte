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
        deleteNote, // Import the deleteNote command
    } from "@services/internal/api/tauri-commands"; // Ensure deleteNote is exposed here
    import { onMount, onDestroy } from "svelte";
    import type { Note } from "@services/internal/api/models/rust-models";
    import { get } from "svelte/store";
    import { addToast } from "@stores/toast-store";
    import { Menu } from "@tauri-apps/api/menu";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event"; // Import listen for menu events
    import ContextualMenu from "@components/atoms/ContextualMenu.svelte";

   let notes: Note[] = $state([]);
  let loadingNotes = $state(false);

  // State for the custom context menu
  let customMenuVisible = false;
  let customMenuX = 0;
  let customMenuY = 0;
  let currentRightClickedNote: Note | null = null; // Store the note that was right-clicked

  // Define menu items specifically for a note context (for the custom menu)
  const noteContextMenuItems = [
    { label: 'Edit Note', action: (id: string) => handleNoteAction('edit', id) },
    { label: 'Delete Note', action: (id: string) => handleNoteAction('delete', id) },
    { label: 'Share Note', action: (id: string) => handleNoteAction('share', id) },
  ];

  // Single, reusable async function to load notes
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

  // Call the new function from the effect
  $effect(() => {
    loadNotesForSpace($activeSpaceName);
  });

  // Load persisted state when the component mounts
  onMount(() => {
    loadPersistentState();
  });

  // Handle actions from the custom context menu
  async function handleNoteAction(actionType: string, noteName: string) {
    console.log(`[Custom Menu Action] ${actionType} for note: ${noteName}`);

    switch (actionType) {
      case 'edit':
        // Implement edit logic, e.g., navigate to editor with this note
        console.log(`[Action] Editing note: ${noteName}`);
        break;
      case 'delete':
        if (noteName && $activeSpaceName) {
          console.log(`[Action] Attempting to delete note: '${noteName}' from space: '${$activeSpaceName}'`);
          try {
            const success = await deleteNote($activeSpaceName, noteName);
            if (success) {
              addToast(`Note '${noteName}' deleted successfully.`, 'success');
              await loadNotesForSpace($activeSpaceName); // Reload notes to update UI
              if (get(activeNote)?.name === noteName) {
                  activeNote.set(null); // Clear active note if it was the deleted one
              }
            } else {
              addToast(`Failed to delete note '${noteName}'.`, 'error');
              console.warn(`[Action] Failed to delete note '${noteName}'.`);
            }
          } catch (error) {
            addToast(`Error deleting note: ${error}`, 'error');
            console.error('[Action] Error calling delete_note command:', error);
          }
        } else {
          console.warn("[Action] No note name or active space for deletion.");
        }
        break;
      case 'share':
        console.log(`[Action] Sharing note: ${noteName}`);
        break;
      default:
        console.log(`[Action] Unimplemented custom menu action: ${actionType}`);
    }
    customMenuVisible = false; // Always close menu after action
  }

  function selectNote(note: Note) {
    console.log("Selecting note:", note);
    activeNote.set(note);
  }

  async function createNote(spaceName: string) {
    let newNoteTitle = "Sin titulo";
    let counter = 1;
    let finalNoteName = `${newNoteTitle} ${counter}`;

    while (notes.some(note => note.name === finalNoteName)) {
      counter++;
      finalNoteName = `${newNoteTitle} ${counter}`;
    }

    try {
      const newNote = await createNoteInSpace(spaceName, { name: finalNoteName, content: "" });
      activeNote.set(newNote);
      await loadNotesForSpace(spaceName);
    } catch (e) {
      addToast(e, 'error')
      console.error("Failed to create note:", e);
    }
  }

  // Function to show the custom context menu for a specific note
  function showNoteContextMenu(event: MouseEvent, note: Note) {
    event.preventDefault(); // Prevent the default browser context menu
    customMenuX = event.clientX;
    customMenuY = event.clientY;
    currentRightClickedNote = note; // Store the note that was right-clicked
    customMenuVisible = true; // Show the custom menu
    console.log(`[ContextMenu] Right-clicked note:`, note.name);
  }

  // Function to close the custom context menu
  function closeCustomContextMenu() {
    customMenuVisible = false;
    currentRightClickedNote = null; // Clear the stored note
  }
</script>

{#if $showWorkspace}
    <div
        class="flex flex-col space-y-md px-lg py-lg bg-black"
        transition:slide={{ duration: 200, axis: "x" }}
    >
        <div class="flex items-center justify-between w-50">
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
                    <Icon iconName="ai" width="20" />
                    <p>Ask Wooly</p>
                </li>
                <li class="text-black-400 flex space-x-xs items-center">
                    <Icon iconName="templates" width="20" />
                    <p>Templates</p>
                </li>
            </ul>
        </div>
        <div class="mt-md">
            <IconNavigationBar intent="horizontal" horizontalSpacing="md">
                <Button
                    intent="icon"
                    handleClick={() => createNote($activeSpaceName)}
                >
                    <Icon iconName="newnote" width="20" />
                </Button>
                <Button intent="icon">
                    <Icon iconName="newfolder" width="20" />
                </Button>
            </IconNavigationBar>
            <div class="mt-md">
                {#each notes as note}
                <li class="list-none" oncontextmenu={(e) => showNoteContextMenu(e, note)}>
                        <Button
                            intent="notes"
                            handleClick={() => selectNote(note)}
                            class={`${$activeNote?.name === note.name ? "text-brand-primary-light bg-black-200" : ""}`}
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
      contextId={currentRightClickedNote?.name || ''}
      on:close={closeCustomContextMenu}
    />
  {/if}