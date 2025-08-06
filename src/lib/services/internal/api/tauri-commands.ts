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
    return invoke<Note[]>('list_notes_in_space_cmd', { spaceName });
}

// Corresponds to `Notes_in_space_cmd` in Rust
export async function createNoteInSpace(spaceName: string, req: CreateNoteRequest): Promise<Note> {
    return invoke<Note>('create_note_in_space_cmd', { spaceName, req });
}

// Corresponds to `get_note_content_cmd` in Rust
export async function getNoteContent(spaceName: string, noteName: string): Promise<NoteContentResponse> {
    return invoke<NoteContentResponse>('get_note_content_cmd', { spaceName, noteName });
}

// Corresponds to `update_note_content_cmd` in Rust
export async function updateNoteContent(spaceName: string, noteName: string, req: UpdateNoteRequest): Promise<void> {
    return invoke('update_note_content_cmd', { spaceName, noteName, req });
}

export async function saveNoteContent(spaceName: string, noteName: string, content: string): Promise<void> {
    return invoke('save_note_content', { spaceName, noteName, content });
}

export async function deleteNote(spaceName: string, noteName: string): Promise<void> {
    return invoke('delete_note', {
        spaceName,
        noteName
    });
}