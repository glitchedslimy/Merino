import { invoke } from "@tauri-apps/api/core";
import { listFolders } from "../../api/tauri/get/folders-api-get";
import { currentRightClickedFolder, folders, inputRenameFolderElement, renamedFolder } from "../../stores/workspace/folders-store";
import type { Folder } from "../../api/tauri/interfaces/folders-interface";
import { contextMenuVisible, contextMenuX, contextMenuY, folderContextMenuVisible, folderContextMenuX, folderContextMenuY, folderRightClicked } from "../../stores/contextmenu/contextmenu-store";
import { get } from "svelte/store";
import { activeSpace } from "../../stores/workspace/spaces-store";
import { createNote, loadNotesInSpace } from "./notes";
import { createFolderInSpace } from "../../api/tauri/create/folder-api-create";
import { toasts } from "../../stores/notifications/toast-store";
import { tick } from "svelte";

let loadingFolders = false;
export async function loadFoldersInSpace(spaceName: string | null) {
    if (!spaceName) {
        folders.set([]);
        loadingFolders = false;
    }
    loadingFolders = true;
    try {
        const loadedFolders = await listFolders(spaceName);
        folders.set(loadedFolders);
        console.log(loadedFolders);
    } catch (e) {
        console.error("Folders not found: ", e);
    } finally {
        loadingFolders = false;
    }
}

export async function moveNoteToFolder(spaceName: string | null, noteName: string, oldFolder: string | null | undefined, newFolder: string | null) {
    const oldFolderValue = (oldFolder === undefined || oldFolder === '') ? null : oldFolder;
    
    await invoke('update_note_route_cmd', { spaceName, noteName, oldFolder: oldFolderValue, newFolder });
}

export async function moveFolderToFolder(spaceName: string | null, folderName: string, oldRoute: string, newRoute: string | null) {
    await invoke('update_folder_route_cmd', { spaceName, folderName, oldRoute, newRoute });
}

async function deleteFolder(spaceName: string | null, folderName: string, folderPath: string | null) {
    return await invoke("delete_folder_cmd", { spaceName, folderName, folderPath });
}

export async function handleFolderActionContextMenu(actionType: string): Promise<void> {
    const folder = get(folderRightClicked);
    if (!folder) return;

    switch (actionType) {
        case "delete":
            if (get(activeSpace)) {
                try {
                    const success = await deleteFolder(get(activeSpace), folder.name, folder.path);
                    if (success) {
                        toasts.add(`"${folder.name} has been deleted."`, "success")
                        await loadNotesInSpace(get(activeSpace))
                    } else {
                        toasts.add(`Couldn't delte the note: ${folder.name}`, "error")
                    }
                } catch (e) {
                    console.error("Error calling 'folder' command: ", e);
                }
            }
            break;
        case "rename":
            renamedFolder.set(folder);
            tick().then(() => {
                const element = get(inputRenameFolderElement);
                if (element) {
                    element.focus();
                }
            })
            break;
        case "create_note":
            console.log(get(folderRightClicked)?.path)
            await createNote(get(activeSpace) ?? '', get(folderRightClicked)?.path ?? '')
            toasts.add(`Created note in folder "${get(folderRightClicked)?.path}" in space.`, "success")
            break;
        case "create_folder":
            await createFolder(get(activeSpace) ?? '', get(folderRightClicked)?.path ?? '')
            toasts.add(`Created folder in folder "${get(folderRightClicked)?.path}" in space.`, "success")
            break;
    }
}

export function showFolderContextmenu(
    event: MouseEvent,
    folderName: string,
    folderPath: string,
) {
    event.preventDefault();
    contextMenuVisible.set(false);
    folderRightClicked.set({ name: folderName, path: folderPath });
    folderContextMenuX.set(event.clientX);
    folderContextMenuY.set(event.clientY);
    folderContextMenuVisible.set(true);
    console.log(get(folderContextMenuVisible))
}

export function closeFolderContextMenu() {
    folderContextMenuVisible.set(false);
    folderRightClicked.set(null);
}

export async function createFolder(spaceName: string, folderPath: string | null) {
    if (!spaceName) {
        return;
    }

    try {
        await createFolderInSpace(spaceName, folderPath);
        loadNotesInSpace(spaceName);
    } catch (e) {
        console.error("Failed to create folder: ", e);
    }
}

export async function renameFolder(newFolderName: string) {
    const space = get(activeSpace);
    const renamed = get(renamedFolder);
    if (!space || !renamed) return;

    if (newFolderName.trim() === "" || newFolderName === renamed.name) {
        renamedFolder.set(null);
        return;
    }

    try {
        await invoke('update_folder_name_cmd', {
            spaceName: space,
            folderName: renamed.name,
            newFolderName,
            folderPath: getParentPath(renamed.path) || null
        });

        loadNotesInSpace(space);
    } catch (e) {
        console.error("Failed to rename folder:", e);
    } finally {
        renamedFolder.set(null);
    }
}

function getParentPath(fullPath: string): string | null {
    if (!fullPath || fullPath.indexOf("/") === -1) {
        return null;
    }
    const parts = fullPath.split("/");
    parts.pop();
    return parts.join("/");
}