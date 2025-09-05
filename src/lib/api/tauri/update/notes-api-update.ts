import { invoke } from "@tauri-apps/api/core";

export async function renameNoteInSpace(spaceName: string | null, noteName: string, newNoteName: string, folderPath: string) {
    return await invoke<any>('update_note_name_cmd', { spaceName, noteName, newNoteName, folderPath })
}

export async function updateNoteContent(spaceName: string, noteName: string, content: number[], folderPath: string | null): Promise<void> {
    return await invoke("update_note_content_cmd", { spaceName, noteName, content, folderPath });
}
