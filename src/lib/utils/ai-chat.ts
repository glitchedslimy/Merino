// ai-chat.ts
import ollama, { type ChatResponse } from 'ollama';

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
        const response = await ollama.list();
        return response.models.map(model => model.name);
    } catch (error) {
        console.error("Failed to fetch Ollama models:", error);
        return [];
    }
}

// sendToChat function updated to not use AbortController
export async function sendToChat(messages: any[], useTools: boolean, modelName: string, useThinking: boolean): Promise<AsyncIterable<ChatResponse>> {
    const params = {
        model: modelName,
        messages,
        stream: true,
        ...(useTools && { tools: tools }),
        ...(useThinking && { think: true }),
    };

    const response = await ollama.chat(params);
    return response;
}

export { availableFunctions };