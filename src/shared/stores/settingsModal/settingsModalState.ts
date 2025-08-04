import { writable } from "svelte/store";

export const showSettingsModal = writable<boolean>(false);