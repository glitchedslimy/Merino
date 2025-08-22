import { listFolders } from "../../api/tauri/get/folders-api-get";
import { folders } from "../../stores/workspace/folders-store";

let loadingFolders = false;
export async function loadFoldersInSpace(spaceName: string | null) {
    if(!spaceName) {
        folders.set([])
        loadingFolders = false;
    }
    loadingFolders = true;
    try {
        const loadedFolders = await listFolders(spaceName);
        folders.set(loadedFolders);
        console.log(loadedFolders)
    } catch(e) {
        console.error("Folders not found: ", e)
    } finally {
        loadingFolders = false;
    }
}