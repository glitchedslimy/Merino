import { writable } from "svelte/store";
import type { Space } from "../../api/tauri/interfaces/spaces-interface";
import { load } from '@tauri-apps/plugin-store';

export const spacesStore = writable<{ spaces?: Space[]}>({
    spaces: []
});

/**
 * Variable to don't load the persistent state always, just on init.
 */
let isInitialLoadComplete = false;

export const activeSpace = writable<string | null>(null);

/**
 * Do the loadPersistent state to load the space that the user has chosen.
 */
const store = load('merino-persistent-state.json', { autoSave: true });
const STORE_KEY = 'activeNameSpace';
export async function loadPersistentSpaceNameState() {
    try {
        const getSpaceName: string | undefined = await (await store).get(STORE_KEY);
        if (getSpaceName) {
            activeSpace.set(getSpaceName);
        }
    } catch(e) {
        console.error("Failed to load persisten store.", e);
    } finally {
        isInitialLoadComplete = true;
    }
}

/**
 * Save the spaceName in the store
 */
export async function saveActiveNameSpace(name: string | null) {
    if(name) {
        (await store).set(STORE_KEY, name);
    } else {
        (await store).delete(STORE_KEY);
    }
    (await store).save();
}

activeSpace.subscribe(async (name) => {
    if (isInitialLoadComplete) {
        await saveActiveNameSpace(name);
    }
})