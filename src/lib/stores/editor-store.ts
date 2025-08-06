import { writable } from "svelte/store";

export const readOnly = writable<boolean>(false);