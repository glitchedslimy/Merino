import { invoke } from "@tauri-apps/api/core";
import { activeTheme, primaryColor } from "../../stores/settings/settings";
import { get } from "svelte/store";

export async function loadSettings() {
    const styleElement = document.createElement("style");
    styleElement.id = "dynamic-theme";
    document.head.appendChild(styleElement);
    try {
        const settingsStr: string = await invoke("get_settings_cmd");
        if (settingsStr) {
            const settings = JSON.parse(settingsStr);
            if (settings.primaryColor) {
                primaryColor.set(settings.primaryColor);
            }
            if (settings.theme) {
                activeTheme.set(settings.theme);
            }
        }
        document.documentElement.setAttribute("data-theme", get(activeTheme) as string);
        document.documentElement.style.setProperty('--color-brand-primary', get(primaryColor));

        (async () => {
            if (get(activeTheme) as string === "default") {
                styleElement.innerHTML = ""; // Clear custom theme styles
            } else {
                try {
                    const themeCss: string = await invoke(
                        "get_theme_content_cmd",
                        { themeName: get(activeTheme) as string },
                    );
                    styleElement.innerHTML = themeCss;
                } catch (e) {
                    console.error("Failed to load theme content:", e);
                    styleElement.innerHTML = "";
                }
            }
        })();
    } catch (e) {
        console.error("Failed to load settings", e);
    }
}