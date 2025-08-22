import { getCurrentWindow } from "@tauri-apps/api/window";
import { isMaximized } from "../../stores/window/window";
import { get } from "svelte/store";

export async function updateMaxState() {
    console.log("Triggerred update")
    isMaximized.set(await getCurrentWindow().isMaximized())
}

export async function toggleMaximizeMinimize() {
    const appWindow = getCurrentWindow();
    if(get(isMaximized)) {
        await appWindow.unmaximize()
    } else {
        await appWindow.maximize()
    }
    await updateMaxState();
}