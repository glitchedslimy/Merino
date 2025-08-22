import { invoke } from "@tauri-apps/api/core";
import type { Note } from "../../../types/notes";
import type { NoteContentResponse } from "../interfaces/notes-interface";

export async function listNotes(spaceName: string): Promise<Note[]> {
    return await invoke<Note[]>("get_notes_in_space_cmd", { spaceName });
}

export async function getNoteContent(spaceName: string, noteName: string): Promise<NoteContentResponse> {
  return await invoke("get_note_content_cmd", { spaceName, noteName });
}