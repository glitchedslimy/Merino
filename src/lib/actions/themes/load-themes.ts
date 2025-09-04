import { invoke } from "@tauri-apps/api/core";

export async function loadThemes() {
    try {
        await invoke("get_themes_cmd")
    } catch(e){
        console.error(e)
    }
}