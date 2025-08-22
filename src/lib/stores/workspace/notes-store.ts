import { writable } from "svelte/store";
import type { Note } from "../../types/notes";

export const notes = writable<Note[]>([]);
export const opennedNotes = writable<Note[]>([]);
export const activeNoteName = writable<string | null>(null);
export const currentRightClickedNote = writable<Note | null>(null);
export const renamedNoteName = writable<string | null>(null);

// InputElement for note rename
export const inputRenameNoteElement = writable<HTMLInputElement | null>(null);