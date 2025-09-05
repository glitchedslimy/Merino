import { invoke } from "@tauri-apps/api/core";

export async function loadThemes() {
    try {
        const themes: string[] = await invoke("get_themes_cmd");
        // Add a "default" option to the beginning of the list
        return [{name: "default"}, ...themes];
    } catch (e) {
        console.error("Failed to load themes:", e);
        return [{name: "default"}];
    }
}
