import { get } from "svelte/store";
import type { Note } from "../../types/notes";
import { activeNoteFolder, activeNoteName, opennedNotes } from "../../stores/workspace/notes-store";

export function openNote(note: Note) {
    const currentNotes = get(opennedNotes);
    // Check if the note is already open by its unique identifier (name + folder)
    const isNoteAlreadyOpen = currentNotes.some(n => n.name === note.name && n.folder === note.folder);

    if (!isNoteAlreadyOpen) {
        opennedNotes.update(notes => [...notes, note]);
    }

    // Set the active note name and folder
    activeNoteName.set(note.name);
    activeNoteFolder.set(note.folder);
}

export function closeNote(noteName: string, noteFolder: string | null) {
    const currentOpenNotes = get(opennedNotes);
    // Filter out the note by its unique identifier (name + folder)
    const newOpenNotes = currentOpenNotes.filter(
        note => !(note.name === noteName && note.folder === noteFolder)
    );

    opennedNotes.set(newOpenNotes);

    // If the closed note was the active one, select the last remaining one
    if (get(activeNoteName) === noteName && get(activeNoteFolder) === noteFolder) {
        const lastNote = newOpenNotes[newOpenNotes.length - 1];
        if (lastNote) {
            activeNoteName.set(lastNote.name);
            activeNoteFolder.set(lastNote.folder);
        } else {
            activeNoteName.set(null);
            activeNoteFolder.set(null);
        }
    }
}