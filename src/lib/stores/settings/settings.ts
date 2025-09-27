import { writable } from "svelte/store";

export const openSettings = writable<boolean>(false);
export const primaryColor = writable<string | null>("#D1600A");
export const activeTheme = writable<string | null>(null);
export const showSpaceCreation = writable<boolean>(false);
export const showModelCreation = writable<boolean>(false);
