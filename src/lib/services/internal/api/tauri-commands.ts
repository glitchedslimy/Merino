// src/lib/api/tauri-commands.ts
import { invoke } from '@tauri-apps/api/core';
import type {
    Space,
    Note,
    CreateSpaceRequest,
    CreateNoteRequest,
    NoteContentResponse,
    UpdateNoteRequest
} from './models/rust-models';

// Corresponds to `list_spaces_cmd` in Rust
export async function listSpaces(): Promise<Space[]> {
    return invoke<Space[]>('list_spaces_cmd');
}

// Corresponds to `create_space_cmd` in Rust
export async function createSpace(req: CreateSpaceRequest): Promise<Space> {
    return invoke<Space>('create_space_cmd', { req });
}

// Corresponds to `delete_space_cmd` in Rust
export async function deleteSpace(spaceName: string): Promise<void> {
    return invoke('delete_space_cmd', { spaceName });
}

// Corresponds to `list_notes_in_space_cmd` in Rust
export async function listNotesInSpace(spaceName: string): Promise<Note[]> {
    console.log("Rust Spacename", spaceName)
    let notes = await invoke("list_notes_in_space_cmd", { spaceName });
    console.log("Notes in Rust", notes)
    return notes as Note[];
}

// Corresponds to `Notes_in_space_cmd` in Rust
export async function createNoteInSpace(spaceName: string, req: CreateNoteRequest): Promise<Note> {
  return await invoke("create_note_in_space_cmd", { spaceName, req });
}

// Corresponds to `get_note_content_cmd` in Rust
export async function getNoteContent(spaceName: string, noteId: string): Promise<NoteContentResponse> {
  return await invoke("load_note_content", { spaceName, noteId });
}

// Corresponds to `update_note_content_cmd` in Rust
export async function updateNoteContent(spaceName: string, noteId: string, req: UpdateNoteRequest): Promise<void> {
    return await invoke("update_note_content_cmd", { spaceName, noteId, req });
}

export async function saveNoteContent(spaceName: string, noteId: string, noteName: string, content: string): Promise<void> {
    return invoke('save_note_content', { spaceName, noteId, noteName, content });
}

export async function deleteNote(spaceName: string, noteId: string): Promise<void> {
      return await invoke("delete_note", { spaceName, noteId });

}

export async function renameNote(spaceName: string, noteId: string, newName: string) {
    return await invoke('rename_note', { spaceName, noteId, newName });
}