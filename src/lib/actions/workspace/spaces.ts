import { getSpaces } from "../../api/tauri/get/spaces-api-get";
import { spacesStore } from "../../stores/workspace/spaces-store";

export async function loadSpaces() {
    try {
        const loadedSpaces = await getSpaces();
        spacesStore.set({ spaces: loadedSpaces })
    } catch(e) {
        console.error("Failed to get spaces: ", e);
    }
}