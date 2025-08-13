// ai-chat.ts
import { invoke } from '@tauri-apps/api/core';

// Mock function for tool calling
const availableFunctions = {
    get_current_weather: async (location: string) => {
        if (location.toLowerCase().includes('san francisco')) {
            return JSON.stringify({ location: "San Francisco", temperature: "72", forecast: "Sunny" });
        }
        return JSON.stringify({ location: location, temperature: "N/A", forecast: "Unknown" });
    }
};

const tools = [
    {
        type: "function",
        function: {
            name: "get_current_weather",
            description: "Get the current weather for a location.",
            parameters: {
                type: "object",
                properties: {
                    location: {
                        type: "string",
                        description: "The location to get the weather for, e.g., San Francisco, CA",
                    },
                },
                required: ["location"],
            },
        },
    },
];

export async function getOllamaModels(): Promise<string[]> {
    try {
        // Use Tauri's invoke to call the Rust command
        const models = await invoke("get_ollama_models_cmd");
        console.log(models)
        return models as string[];
    } catch (error) {
        console.error("Failed to fetch Ollama models:", error);
        return [];
    }
}

export async function sendToChat(messages: any[], useTools: boolean, modelName: string, useThinking: boolean): Promise<void> {
    // Call the Rust command, which will handle the streaming on the backend.
    // We pass all the necessary parameters to the backend.
    await invoke("send_to_chat_command", {
        messages,
        model: modelName,
        useTools,
        useThinking,
    });
}

// A new function to invoke a backend command to stop the streaming.
export async function stopStreaming(): Promise<void> {
    await invoke("stop_ollama_stream");
}

export { availableFunctions };