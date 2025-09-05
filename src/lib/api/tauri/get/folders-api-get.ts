import { invoke } from "@tauri-apps/api/core";
import type { Folder } from "../interfaces/folders-interface";

export async function listFolders(spaceName: string | null): Promise<Folder[]> {
    return await invoke<Folder[]>("get_folders_in_space_cmd", { spaceName });
}