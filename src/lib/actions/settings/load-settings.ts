import { invoke } from "@tauri-apps/api/core";
import { primaryColor } from "../../stores/settings/settings";
import { get } from "svelte/store";

export async function loadSettings() {
        try {
            const settingsStr: string = await invoke("get_settings_cmd");
            console.log(settingsStr)
            if (settingsStr) {
                const settings = JSON.parse(settingsStr);
                if (settings.primaryColor) {
                    primaryColor.set(settings.primaryColor);
                }
            }
             document.documentElement.style.setProperty('--color-brand-primary', get(primaryColor));
        } catch (e) {
            console.error("Failed to load settings", e);
        }
    }