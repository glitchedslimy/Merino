import { writable } from "svelte/store";

export const showTooltip = writable<boolean>(false);
export const tooltipText = writable<string>("");
export const tooltipX = writable<number>(0);
export const tooltipY = writable<number>(0);