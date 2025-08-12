import type { Space } from "@services/internal/api/models/rust-models";
import { writable } from "svelte/store";
import { load } from "@tauri-apps/plugin-store";
import type { OutputData } from "@editorjs/editorjs";

export const showWorkspace = writable<boolean>(true);
let isInitialLoadComplete = false;

export const spacesStore = writable<{ spaces: Space[], activeSpaceName: string | null }>({
    spaces: [],
    activeSpaceName: null,
});

export const openAdminSpaces = writable(false);

const store = load('notalia-states.json', { autoSave: true });
const STORE_KEY = 'activeSpaceName';

export const activeSpaceName = writable<string | null>(null);

export async function loadPersistentState() {
    try {
        const saveSpaceName = await (await store).get(STORE_KEY)
        console.log(saveSpaceName)
        if(saveSpaceName) {
            activeSpaceName.set(saveSpaceName);
        }
    } catch(e) {
        console.error("Failed to load persistent store.", e)
    } finally {
        isInitialLoadComplete = true;
    }
}

export async function saveActiveSpaceName(name: string) {
    if(name) {
        (await store).set(STORE_KEY, name);
    } else {
        (await store).delete(STORE_KEY);
    }
    (await store).save()
}

activeSpaceName.subscribe(async (name) => {
    // Only save if the initial load is complete
    if (isInitialLoadComplete) {
        await saveActiveSpaceName(name);
    }
})

export const openNotes = writable([]);
export const activeNoteId = writable<string | null>(null);