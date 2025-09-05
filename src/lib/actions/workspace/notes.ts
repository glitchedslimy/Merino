import { get } from "svelte/store";
import { listNotes } from "../../api/tauri/get/notes-api-get";
import { activeNoteFolder, activeNoteName, currentRightClickedNote, inputRenameNoteElement, notes, opennedNotes, renamedNote } from "../../stores/workspace/notes-store";
import { activeSpace } from "../../stores/workspace/spaces-store";
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "../../types/notes";
import { contextMenuVisible, contextMenuX, contextMenuY, folderContextMenuVisible } from "../../stores/contextmenu/contextmenu-store";
import { createNoteInSpace } from "../../api/tauri/create/notes-api-create";
import { renameNoteInSpace } from "../../api/tauri/update/notes-api-update";
import { tick } from "svelte";
import { showTooltip, tooltipText, tooltipX, tooltipY } from "../../stores/tooltip/tooltip-store";
import { closeNote, openNote } from "../editor/notes-buffer";
import { toasts } from "../../stores/notifications/toast-store";
import { listFolders } from "../../api/tauri/get/folders-api-get";

let loadingNotes = false;
export async function loadNotesInSpace(spaceName: string | null) {
    if (!spaceName) {
        notes.set([]);
        loadingNotes = false;
        opennedNotes.set([]);
        activeNoteName.set(null);
        activeNoteFolder.set(null);
        return;
    }
    loadingNotes = true;
    try {
        const loadedNotes = await listNotes(spaceName);
        const loadedFolders = await listFolders(spaceName);

        // FIX: Pass both the notes and folders arrays to the buildNoteTree function
        const notesTree = buildNoteTree(loadedNotes, loadedFolders);
        notes.set(notesTree);
        if (loadedNotes.length > 0 && get(opennedNotes).length === 0) {
            
        }
    } catch (e) {
        console.error("Failed to load notes: ", e);
        notes.set([]);
        opennedNotes.set([]);
        activeNoteName.set(null);
        activeNoteFolder.set(null);
    } finally {
        loadingNotes = false;
    }
}

async function deleteNote(spaceName: string | null, noteName: string, folderPath: string | null) {
    return await invoke("delete_note_cmd", { spaceName, noteName, folderPath });
}

export async function handleNoteActionContextMenu(actionType: string): Promise<void> {
    const note = get(currentRightClickedNote);
    if (!note) return;

    switch (actionType) {
        case "delete":
            if (get(activeSpace)) {
                try {
                    const success = await deleteNote(get(activeSpace), note.name, note.folder);
                    if (success) {
                        toasts.add(`"${note.name}" has been deleted.`, "success");
                        closeNote(note.name, note.folder);
                        await loadNotesInSpace(get(activeSpace));
                    } else {
                        toasts.add(`Couldn't delete the note: "${note.name}"`, "error");
                    }
                } catch (e) {
                    console.error("Error calling 'delete_note' command: ", e);
                }
            }
            break;
        case "rename":
            renamedNote.set(note);
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
    event.preventDefault();
    folderContextMenuVisible.set(false);
    contextMenuX.set(event.clientX);
    contextMenuY.set(event.clientY);
    currentRightClickedNote.set(note);
    contextMenuVisible.set(true);
}

export function closeContextMenu() {
    contextMenuVisible.set(false);
    currentRightClickedNote.set(null);
}

export async function createNote(spaceName: string, folderPath: string | null) {
    if (!spaceName) {
        return;
    }

    try {
        const newNote = await createNoteInSpace(spaceName, folderPath);

        // Add the new note to the opennedNotes store
        opennedNotes.update(currentNotes => {
            const isNoteOpen = currentNotes.some(note => note.name === newNote.name && note.folder === newNote.folder);
            if (!isNoteOpen) {
                return [...currentNotes, newNote];
            }
            return currentNotes;
        });
        openNote(newNote)
        loadNotesInSpace(spaceName)
    } catch (e) {
        console.error("Failed to create note: ", e);
    }
}

export async function renameNote(newName: string) {
    const oldNote = get(renamedNote);
    if (!newName.trim() || !oldNote) {
        renamedNote.set(null);
        return;
    }

    if (get(activeSpace)) {
        try {
            const renamedNote = await renameNoteInSpace(get(activeSpace), oldNote.name, newName, oldNote.folder);

            if (renamedNote) {
                opennedNotes.update((notes) => {
                    const noteIndex = notes.findIndex((n) => n.name === oldNote.name);
                    if (noteIndex !== -1) {
                        notes[noteIndex] = renamedNote;
                    }
                    return notes;
                });
                if (get(activeNoteName) === oldNote.name) {
                    openNote(renamedNote);
                }
                await loadNotesInSpace(get(activeSpace));
            } else {
                toasts.add("Error trying to rename the note.", "error");
            }
        } catch (e) {
            console.error("Error renaming the note: ", e);
        }
    }
    renamedNote.set(null);
}

let timeoutId: number | null = null;
const DELAY_MS = 0;

export function handleShowTooltip(text: string, x: number, y: number) {
    if (timeoutId) {
        clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => {
        showTooltip.set(true);
        tooltipText.set(text);
        tooltipX.set(x);
        tooltipY.set(y);
    }, DELAY_MS);
}

export function handleHideTooltip() {
    if (timeoutId) {
        clearTimeout(timeoutId);
        timeoutId = null;
    }
    showTooltip.set(false);
    tooltipText.set("");
}

export function selectNote(note: Note) {
    openNote(note);
}

export function buildNoteTree(notes: any[], folders: { path: string }[]) {
    const tree = {
        notes: [],
        folders: {}
    };

    folders.forEach(folder => {
        const folderPath = folder.path;
        
        if (typeof folderPath !== 'string' || folderPath.trim() === '') {
            return;
        }

        let currentLevel = tree.folders;
        const pathSegments = folderPath.split(/[\\\/]/);
        
        pathSegments.forEach(segment => {
            if (segment === '') {
                return;
            }
            if (!currentLevel[segment]) {
                currentLevel[segment] = {
                    notes: [],
                    folders: {}
                };
            }
            currentLevel = currentLevel[segment].folders;
        });
    });

    notes.forEach(note => {
        if (!note.folder) {
            tree.notes.push(note);
            return;
        }

        if (typeof note.folder !== 'string' || note.folder.trim() === '') {
            tree.notes.push(note);
            return;
        }

        const pathSegments = note.folder.split(/[\\\/]/);
        let currentLevel = tree.folders;

        pathSegments.forEach((segment, index) => {
            if (index === pathSegments.length - 1) {
                if (currentLevel[segment]) {
                    currentLevel[segment].notes.push(note);
                }
            } else {
                currentLevel = currentLevel[segment].folders;
            }
        });
    });

    return tree;
}