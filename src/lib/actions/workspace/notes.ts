import { get } from "svelte/store";
import { listNotes } from "../../api/tauri/get/notes-api-get";
import { activeNoteName, currentRightClickedNote, inputRenameNoteElement, notes, opennedNotes, renamedNoteName } from "../../stores/workspace/notes-store";
import { activeSpace } from "../../stores/workspace/spaces-store";
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "../../types/notes";
import { contextMenuVisible, contextMenuX, contextMenuY } from "../../stores/contextmenu/contextmenu-store";
import { createNoteInSpace } from "../../api/tauri/create/notes-api-create";
import { renameNoteInSpace } from "../../api/tauri/update/notes-api-update";
import { tick } from "svelte";
import { showTooltip, tooltipText, tooltipX, tooltipY } from "../../stores/tooltip/tooltip-store";
import { closeNote, openNote } from "../editor/notes-buffer";
import { notifications } from "../../stores/notifications/notification-store";
import { toasts } from "../../stores/notifications/toast-store";

let loadingNotes = false;
export async function loadNotesInSpace(spaceName: string | null) {
    if (!spaceName) {
        notes.set([]);
        loadingNotes = false;
        opennedNotes.set([]);
        activeNoteName.set(null);
        return;
    }
    loadingNotes = true;
    try {
        const loadedNotes = await listNotes(spaceName);
        notes.set(loadedNotes);
        if (loadedNotes.length > 0 && get(opennedNotes).length === 0) {
            openNote(loadedNotes[0])
        }
    } catch (e) {
        console.error("Failed to load notes: ", e);
        notes.set([]);
        opennedNotes.set([]),
            activeNoteName.set(null);
    } finally {
        loadingNotes = false;
    }
}

async function deleteNote(spaceName: string | null, noteName: string) {
    return await invoke("delete_note_cmd", { spaceName, noteName })
}

export async function handleNoteActionContextMenu(actionType: string, noteName: string): Promise<void> {
    switch (actionType) {
        case "delete":
            if (noteName && get(activeSpace)) {
                try {
                    const success = await deleteNote(get(activeSpace), noteName)
                    if (success) {
                        toasts.add(`"${noteName}" has been deleted.`, "success")
                        closeNote(noteName)
                        await loadNotesInSpace(get(activeSpace))
                    } else {
                        toasts.add(`Couldn't delete the note: "${noteName}"`, "error")
                    }
                } catch (e) {
                    console.error("Error calling 'delete_note' command: ", e)
                }
            }
            break;
        case "rename":
            renamedNoteName.set(noteName);
            tick().then(() => {
                const element = get(inputRenameNoteElement);
                if (element) {
                    element.focus();
                }
            });
            break;
    }
}

export function showNoteContextmenu(event: MouseEvent, note: Note) {
    event.preventDefault()
    contextMenuX.set(event.clientX);
    contextMenuY.set(event.clientY);
    currentRightClickedNote.set(note);
    contextMenuVisible.set(true);
}

export function closeContextMenu() {
    contextMenuVisible.set(false);
    currentRightClickedNote.set(null)
}

export async function createNote(spaceName: string) {
    try {
        const newNote = await createNoteInSpace(spaceName);
        // Open Note in editor
        await loadNotesInSpace(spaceName)
    } catch (e) {
        toasts.add("Error to create the note.", "error")
        console.error("Failed to create note: ", e);
    }
}

export async function renameNote(oldName: string, newName: string) {
    if (!newName.trim()) {
        renamedNoteName.set(null)
        return;
    }

    if (oldName && get(activeSpace)) {
        try {
            const renamedNote = await renameNoteInSpace(get(activeSpace), oldName, newName);

            if (renamedNote) {
                opennedNotes.update((notes) => {
                    const noteIndex = notes.findIndex((n) => n.name === oldName);
                    if (noteIndex !== -1) {
                        notes[noteIndex] = renamedNote
                    }
                    return notes;
                });
                if (get(activeNoteName) === oldName) {
                    openNote(renamedNote)
                }
                await loadNotesInSpace(get(activeSpace));
            } else {
                toasts.add("Error trying to rename the note.", "error")
            }
        } catch (e) {
            console.error("Error renaming the note: ", e);
        }
    }
    renamedNoteName.set(null)
}

let timeoutId: number | null = null;
const DELAY_MS = 0;

export function handleShowTooltip(text: string, x: number, y: number) {
    console.log("Showing tooltip")
    if (timeoutId) {
        clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => {
    console.log("showing tooltip")
      showTooltip.set(true)
      tooltipText.set(text)
      tooltipX.set(x)
      tooltipY.set(y)
    }, DELAY_MS);
}

export function handleHideTooltip() {
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    showTooltip.set(false)
    tooltipText.set("")
}

export function selectNote(note: Note) {
    openNote(note);
  }