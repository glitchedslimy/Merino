import { writable } from "svelte/store";

export const openSettings = writable<boolean>(false);
export const primaryColor = writable<string | null>("#D1600A");