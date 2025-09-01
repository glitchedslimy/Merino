import { writable } from "svelte/store";

export const openAiChat = writable<boolean>(false);
export const selectedModel = writable<string | null>("");
export const aiMessages = writable<{sender: string, text: string | unknown, thinking?: string, isThinkingDropdownOpen?: boolean}[]>([]);
export const chatContainer = writable<HTMLDivElement | null>(null);
export const aiIsLoading = writable<boolean>(false);