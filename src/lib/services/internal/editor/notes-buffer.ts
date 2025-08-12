import { activeNoteId, openNotes } from "@stores/workspace-store";
import { get } from "svelte/store";
import type { Note } from "../api/models/rust-models";

export function openNote(note) {
    const currentNotes = get(openNotes);
    const isNoteAlreadyOpen = currentNotes.some(n => n.id === note.id);

    if (!isNoteAlreadyOpen) {
        openNotes.update(notes => [...notes, note]);
        console.log("Adding new note to buffer:", note.name);
    }
    
    activeNoteId.set(note.id);
    console.log("Setting active note ID:", note.id);
    console.log("Current open notes in store:", get(openNotes));
}
export function closeNote(noteId) {
    const currentOpenNotes = get(openNotes);
    const newOpenNotes = currentOpenNotes.filter(note => note.id !== noteId);

    openNotes.set(newOpenNotes);

    // If the closed note was the active one, select the last remaining one
    if (get(activeNoteId) === noteId) {
        const lastNote = newOpenNotes[newOpenNotes.length - 1];
        activeNoteId.set(lastNote ? lastNote.id : null);
    }
}