import { get } from "svelte/store";
import type { Note } from "../../types/notes";
import { activeNoteName, opennedNotes } from "../../stores/workspace/notes-store";

export function openNote(note: Note) {
    const currentNotes = get(opennedNotes);
    // Check if the note is already open by its name
    const isNoteAlreadyOpen = currentNotes.some(n => n.name === note.name);

    if (!isNoteAlreadyOpen) {
        opennedNotes.update(notes => [...notes, note]);
        console.log("Adding new note to buffer:", note.name);
    }
    
    // Set the active note name
    activeNoteName.set(note.name);
    console.log("Setting active note name:", note.name);
    console.log("Current open notes in store:", get(opennedNotes));
}

export function closeNote(noteName: string) {
    const currentOpenNotes = get(opennedNotes);
    // Filter out the note by its name
    const newOpenNotes = currentOpenNotes.filter(note => note.name !== noteName);

    opennedNotes.set(newOpenNotes);

    // If the closed note was the active one, select the last remaining one
    if (get(activeNoteName) === noteName) {
        const lastNote = newOpenNotes[newOpenNotes.length - 1];
        activeNoteName.set(lastNote ? lastNote.name : null);
    }
}