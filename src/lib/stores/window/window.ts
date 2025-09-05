import { writable } from "svelte/store";

export const isMaximized = writable<boolean>(false);