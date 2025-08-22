import { writable } from "svelte/store";

export const contextMenuX = writable<number>(0); 
export const contextMenuY = writable<number>(0);
export const contextMenuVisible = writable<boolean>(false);
