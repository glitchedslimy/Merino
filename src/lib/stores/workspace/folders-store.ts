import { writable } from "svelte/store";
import { type Folder } from "../../api/tauri/interfaces/folders-interface";

export const folders = writable<Folder[]>([]);
export const currentRightClickedFolder = writable<Folder | null>(null);
export const renamedFolder = writable<{ name: string; path: string } | null>(null);
export const inputRenameFolderElement = writable<HTMLInputElement | null>(null);
