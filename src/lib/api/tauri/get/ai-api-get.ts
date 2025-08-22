import { invoke } from "@tauri-apps/api/core";

export async function getOllamaModels(): Promise<string[]> {
    try {
        const models = await invoke('get_ai_models_cmd');
        return models as string[];
    } catch(e) {
        return [];
    }
}