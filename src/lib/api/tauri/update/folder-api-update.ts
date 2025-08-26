import { invoke } from "@tauri-apps/api/core";

export async function renameNoteInSpace(spaceName: string | null, folderName: string, newFolderName: string, folderPath: string) {
    return await invoke<any>('update_folder_name_cmd', { spaceName, folderName, newFolderName, folderPath })
}
