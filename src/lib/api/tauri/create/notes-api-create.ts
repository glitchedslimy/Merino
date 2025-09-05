import { invoke } from "@tauri-apps/api/core";

export async function createNoteInSpace(spaceName: string | null, folderPath: string | null) {
    return await invoke("create_note_in_space_cmd", { spaceName, folderPath });
}