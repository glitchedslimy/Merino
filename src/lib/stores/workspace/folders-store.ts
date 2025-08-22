import { writable } from "svelte/store";
import { type Folder } from "../../api/tauri/interfaces/folders-interface";

export const folders = writable<Folder[]>([]);