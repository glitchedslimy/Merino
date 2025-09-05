import { invoke } from "@tauri-apps/api/core";

export async function createFolderInSpace(spaceName: string | null, folderPath: string | null) {
    return await invoke("create_folder_cmd", { spaceName, folderPath });
}