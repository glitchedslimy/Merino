import { writable } from 'svelte/store';
import type { Note } from '../../types/notes';

// A writable store for the name of the currently active note.
export const activeNoteName = writable<string | null>(null);

// A writable store for the folder path of the currently active note.
// This is necessary to uniquely identify a note when multiple notes have the same name.
export const activeNoteFolder = writable<string | null>(null);

// A writable store for all notes available in the workspace.
export const notes = writable<Note[]>([]);

// A writable store for the notes that are currently open in tabs.
export const opennedNotes = writable<Note[]>([]);

// A writable store for the note that is currently being renamed.
export const renamedNote = writable<{name: string, content: string | null, folder: string | null} | null>(null);

// A writable store for the element of the input field used for renaming.
export const inputRenameNoteElement = writable<HTMLInputElement | null>(null);

// A writable store for the note that was last right-clicked to show the context menu.
export const currentRightClickedNote = writable<Note | null>(null);