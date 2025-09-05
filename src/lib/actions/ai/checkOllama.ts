import { invoke } from "@tauri-apps/api/core";
import { isOllamaRunning } from "../../stores/ai/ai-store";

export async function checkOllama() {
        try {
            let status: boolean = await invoke("check_ollama_status_cmd");
            isOllamaRunning.set(status);
        } catch (e) {
            console.error(e);
            isOllamaRunning.set(false);
        }
    }