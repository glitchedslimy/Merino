// src/lib/api/models.ts

// Matches the Rust `Space` struct
export interface Space {
    name: string;
    route: string | null, 
}

// Matches the Rust `Note` struct
export interface Note {
    name: string;
}

// Matches the Rust `CreateSpaceRequest`
export interface CreateSpaceRequest {
    name: string;
}

// Matches the Rust `CreateNoteRequest`
export interface CreateNoteRequest {
    name: string;
    content: string;
}

// Matches the Rust `NoteContentResponse`
export interface NoteContentResponse {
    name: string;
    content: string;
}

// Matches the Rust `UpdateNoteRequest`
export interface UpdateNoteRequest {
    content: string;
}