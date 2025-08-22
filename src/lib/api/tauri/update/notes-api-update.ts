import { invoke } from "@tauri-apps/api/core";

export async function renameNoteInSpace(spaceName: string | null, noteName: string, newNoteName: string) {
    return await invoke<any>('update_note_name_cmd', { spaceName, noteName, newNoteName })
}

export async function updateNoteContent(spaceName: string, noteName: string, content: number[]): Promise<void> {
    return await invoke("update_note_content_cmd", { spaceName, noteName, content });
}
