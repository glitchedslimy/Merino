import { writable } from "svelte/store";

export const contextMenuX = writable<number>(0); 
export const contextMenuY = writable<number>(0);
export const contextMenuVisible = writable<boolean>(false);


export const folderContextMenuVisible = writable(false);
export const folderContextMenuX = writable(0);
export const folderContextMenuY = writable(0);
export const folderRightClicked = writable<{ name: string, path: string } | null>(null);
