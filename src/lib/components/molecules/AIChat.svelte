<script lang="ts">
    import ollama, { type ChatResponse } from "ollama";
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import Markdown from "@components/molecules/Markdown.svelte";
    import { openAiChat } from "@stores/ai-store";
    import { slide } from "svelte/transition";
    import {
        sendToChat,
        availableFunctions,
        getOllamaModels,
    } from "../../utils/ai-chat";
    import { onMount } from "svelte";

    function closeChat() {
        openAiChat.set(false);
    }

    let message: string = $state("");
    let conversation: {
        text: string;
        sender: "user" | "ai" | "thinking-steps" | "loading-ai";
        steps?: string[];
        duration?: number;
        expanded?: boolean;
        isThinking?: boolean;
        liveDuration?: number;
    }[] = $state([]);
    let messagesForOllama: any[] = $state([]);
    let mode: "chat" | "agentic" = $state("chat");
    let showModeDropdown: boolean = $state(false);
    let isGenerating: boolean = $state(false);

    let models: string[] = $state([]);
    let selectedModel: string = $state("llama3.1");
    let showModelDropdown: boolean = $state(false);
    let isFetchingModels: boolean = $state(true);

    let useThinking: boolean = $state(false);
    let thinkingInterval: number | null = null;
    let isLoadingAiResponse: boolean = $state(false);

    onMount(async () => {
        isFetchingModels = true;
        models = await getOllamaModels();
        if (models.length > 0) {
            selectedModel = models.includes("llama3.1")
                ? "llama3.1"
                : models[0];
        }
        isFetchingModels = false;
    });

    async function handleUserMessage() {
        if (!message.trim() || isFetchingModels || isGenerating) return;

        isGenerating = true;
        const userMessage = message;
        conversation.push({ text: userMessage, sender: "user" });
        message = "";
        messagesForOllama.push({ role: "user", content: userMessage });

        await processChatResponse();
    }

    function cleanupAfterGeneration() {
        isGenerating = false;
        isLoadingAiResponse = false;
        if (thinkingInterval) {
            clearInterval(thinkingInterval);
            thinkingInterval = null;
        }
    }

    async function processChatResponse() {
        const useTools = mode === "agentic";
        let thinkingMessageIndex = -1;

        if (useThinking) {
            thinkingMessageIndex = conversation.length;
            conversation.push({
                sender: "thinking-steps",
                steps: [],
                duration: 0,
                expanded: false,
                isThinking: true,
                liveDuration: 0,
            });

            const startTime = Date.now();
            thinkingInterval = setInterval(() => {
                if (conversation[thinkingMessageIndex]) {
                    conversation[thinkingMessageIndex].liveDuration =
                        Date.now() - startTime;
                } else {
                    clearInterval(thinkingInterval);
                    thinkingInterval = null;
                }
            }, 100);
        }

        let aiResponseStream: AsyncIterable<ChatResponse>;
        let responseStarted = false;
        let aborted = false;

        try {
            const response = await sendToChat(
                messagesForOllama,
                useTools,
                selectedModel,
                useThinking,
            );
            aiResponseStream = response;
        } catch (error) {
            console.error("Failed to start chat stream:", error);
            cleanupAfterGeneration();
            return;
        }

        let fullContent = "";
        let toolCalls = [];

        try {
            for await (const chunk of aiResponseStream) {
                responseStarted = true;
                if (chunk.message?.thinking) {
                    if (useThinking && thinkingMessageIndex !== -1) {
                        conversation[thinkingMessageIndex].steps.push(
                            chunk.message.thinking,
                        );
                    }
                } else if (chunk.message?.content) {
                    fullContent += chunk.message.content;
                }
                if (chunk.message?.tool_calls) {
                    toolCalls = [...toolCalls, ...chunk.message.tool_calls];
                }
            }
        } catch (error: any) {
            if (error.name === "AbortError") {
                console.log("Stream aborted by user.");
                aborted = true;
            } else {
                console.error("Error during stream processing:", error);
            }
        } finally {
            if (useThinking && thinkingMessageIndex !== -1) {
                if (thinkingInterval) {
                    clearInterval(thinkingInterval);
                    thinkingInterval = null;
                }
                if (conversation[thinkingMessageIndex]) {
                    conversation[thinkingMessageIndex].isThinking = false;
                    const endTime = Date.now();
                    conversation[thinkingMessageIndex].duration =
                        endTime -
                        (conversation[thinkingMessageIndex].liveDuration
                            ? endTime -
                              conversation[thinkingMessageIndex].liveDuration
                            : endTime);
                }
            }

            if (!aborted) {
                if (toolCalls.length > 0) {
                    isLoadingAiResponse = false;

                    const toolCallMessageIndex = conversation.length;
                    conversation.push({
                        sender: "thinking-steps",
                        steps: [`Calling **${toolCalls[0].function.name}**...`],
                        expanded: true,
                        isThinking: true,
                        duration: 0,
                    });

                    const toolOutputs = [];
                    for (const toolCall of toolCalls) {
                        const functionName = toolCall.function.name;
                        const functionToCall = availableFunctions[functionName];

                        if (functionToCall) {
                            const args = toolCall.function.arguments;
                            const functionOutput = await functionToCall(
                                args.location,
                            );
                            toolOutputs.push({
                                tool_call_id: toolCall.id,
                                output: functionOutput,
                            });

                            // Update the single message to 'Called' and add the result
                            conversation[toolCallMessageIndex].steps = [
                                `Called **${toolCalls[0].function.name}**.`,
                                `Tool call successful. Result: ${functionOutput}`,
                            ];
                            conversation[toolCallMessageIndex].isThinking =
                                false;
                        } else {
                            console.error(
                                `Tool function not found: ${functionName}`,
                            );
                            // Handle the error by updating the calling message
                            conversation[toolCallMessageIndex].steps = [
                                `Error calling **${toolCalls[0].function.name}**.`,
                            ];
                            conversation[toolCallMessageIndex].isThinking =
                                false;
                        }
                    }

                    messagesForOllama.push({
                        role: "assistant",
                        content: "",
                        tool_calls: toolCalls,
                    });
                    for (const output of toolOutputs) {
                        messagesForOllama.push({
                            role: "tool",
                            content: output.output,
                            tool_call_id: output.tool_call_id,
                        });
                    }

                    isLoadingAiResponse = true;
                    let finalContent = "";
                    try {
                        const finalResponse = await sendToChat(
                            messagesForOllama,
                            useTools,
                            selectedModel,
                            useThinking,
                        );
                        for await (const chunk of finalResponse) {
                            if (chunk.message?.content) {
                                finalContent += chunk.message.content;
                            }
                        }
                    } catch (error: any) {
                        if (error.name === "AbortError") {
                            console.log("Final stream aborted by user.");
                            aborted = true;
                        } else {
                            console.error(
                                "Error during final stream processing:",
                                error,
                            );
                        }
                    }

                    if (finalContent) {
                        conversation.push({ text: finalContent, sender: "ai" });
                        messagesForOllama.push({
                            role: "assistant",
                            content: finalContent,
                        });
                    }
                } else if (fullContent) {
                    conversation.push({ text: fullContent, sender: "ai" });
                    messagesForOllama.push({
                        role: "assistant",
                        content: fullContent,
                    });
                }
            } else {
                if (responseStarted) {
                    conversation.push({
                        text: "Message generation stopped by user.",
                        sender: "ai",
                    });
                }
            }

            cleanupAfterGeneration();
        }
    }
    // New stopGenerating function using ollama.abort()
    function stopGenerating() {
        ollama.abort();

        // Perform cleanup for the UI
        isGenerating = false;
        isLoadingAiResponse = false;
        if (thinkingInterval) {
            clearInterval(thinkingInterval);
            thinkingInterval = null;
        }

        const lastMessage = conversation[conversation.length - 1];
        if (
            lastMessage &&
            (lastMessage.sender === "thinking-steps" ||
                lastMessage.sender === "loading-ai") &&
            lastMessage.isThinking
        ) {
            lastMessage.isThinking = false;
        }
    }
</script>

{#if $openAiChat}
    <section
        class="bg-black rounded-md w-72 px-md py-md flex flex-col justify-between"
        transition:slide={{ duration: 200, axis: "x" }}
    >
        <div class="flex justify-between items-center mb-sm">
            <Button intent="icon" handleClick={closeChat}>
                <Icon iconName="chevron-pipe-right" width="20" />
            </Button>
            <Icon iconName="WoolyStroke" width="20" />
        </div>

        <div class="flex-1 overflow-y-auto my-md">
            {#each conversation as chatMessage}
                {#if chatMessage.sender === "user"}
                    <div class="mb-sm text-right">
                        <div
                            class="inline-block p-sm rounded-lg bg-brand-primary"
                        >
                            <Markdown text={chatMessage.text} />
                        </div>
                    </div>
                {:else if chatMessage.sender === "ai"}
                    <div class="mb-sm">
                        <div class="inline-block p-sm rounded-lg bg-black-100">
                            <Markdown text={chatMessage.text} />
                        </div>
                    </div>
                {:else if chatMessage.sender === "thinking-steps"}
                    <div class="mb-sm">
                        <button
                            onclick={() =>
                                (chatMessage.expanded = !chatMessage.expanded)}
                            class="flex items-center space-x-sm p-sm rounded-md text-black-200 text-sm w-full text-left"
                        >
                            <Icon
                                iconName="chevron-down"
                                width="16"
                                className={`${chatMessage.expanded ? "rotate-180" : ""}`}
                            />
                            <span>
                                {#if chatMessage.isThinking}
                                    <div class="flex items-center space-x-xs">
                                        <span class="animate-spin loader-icon"
                                        ></span>
                                        <Markdown text={chatMessage.steps[0]} />
                                    </div>
                                {:else}
                                    {(chatMessage.duration / 1000).toFixed(2)}s
                                    - Thought
                                {/if}
                            </span>
                        </button>
                        {#if chatMessage.expanded}
                            <div class="pl-md pt-sm text-gray-400 text-xs">
                                <Markdown text={chatMessage.steps.join(" ")} />
                            </div>
                        {/if}
                    </div>
                {/if}
            {/each}

            {#if isLoadingAiResponse}
                <div class="mb-sm">
                    <div class="inline-block p-sm rounded-lg bg-gray-700">
                        <div class="flex items-center space-x-xs text-white">
                            <span class="animate-spin loader-icon-lg"></span>
                            <span>Loading response...</span>
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        <div class="flex flex-col space-y-sm mt-auto mb-sm">
            <div class="flex items-center justify-between">
                <div class="dropdown-wrapper">
                    <button
                        onclick={() => (showModeDropdown = !showModeDropdown)}
                        class="flex items-center text-white font-bold p-sm rounded-md"
                        title={mode === "chat" ? "Chat Mode" : "Agentic Mode"}
                    >
                        {#if mode === "agentic"}
                            <Icon iconName="stars" width="16" />
                        {:else}
                            <Icon iconName="ai" width="16" />
                        {/if}
                        <Icon iconName="chevron-down" width="16" />
                    </button>
                    {#if showModeDropdown}
                        <div
                            class="absolute z-10 bottom-full bg-black-100 rounded-md"
                            onmouseleave={() => (showModeDropdown = false)}
                        >
                            <button
                                class="block w-full text-left px-sm py-xs hover:bg-black-200 rounded-md flex items-center space-x-sm"
                                onclick={() => {
                                    mode = "chat";
                                    showModeDropdown = false;
                                }}
                            >
                                <Icon iconName="ai" width="20" />
                                <span>Chat Mode</span>
                            </button>
                            <button
                                class="block w-full text-left px-sm py-xs hover:bg-black-200 rounded-md flex items-center space-x-sm"
                                onclick={() => {
                                    mode = "agentic";
                                    showModeDropdown = false;
                                }}
                            >
                                <Icon iconName="stars" width="20" />
                                <span>Tool Mode</span>
                            </button>
                        </div>
                    {/if}
                </div>

                <div class="dropdown-wrapper flex-1 mx-sm">
                    <button
                        onclick={() => (showModelDropdown = !showModelDropdown)}
                        class="flex items-center text-white text-sm font-medium rounded-md space-x-xs justify-between overflow-hidden text-ellipsis whitespace-nowrap"
                        title={isFetchingModels
                            ? "Loading..."
                            : selectedModel || "Select Model"}
                    >
                        <img
                            src="./src/assets/ollama.png"
                            width="15"
                            height="15"
                            alt="Ollama logo"
                            class="invert-100"
                        />
                        <span
                            class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap"
                        >
                            {isFetchingModels
                                ? "Loading..."
                                : selectedModel.split("-")[0].split(".")[0] ||
                                  "Select Model"}
                        </span>
                        <Icon iconName="chevron-down" width="16" />
                    </button>
                    {#if showModelDropdown && models.length > 0}
                        <div
                            class="absolute bottom-full z-10 bg-black-100 rounded-md flex flex-col space-y-sm text-sm"
                            onmouseleave={() => (showModelDropdown = false)}
                        >
                            {#each models as model}
                                <div
                                    class="flex items-center rounded-md hover:bg-black-200 space-x-sm px-sm py-sm cursor-pointer"
                                    onclick={() => {
                                        selectedModel = model;
                                        showModelDropdown = false;
                                    }}
                                    title={model}
                                >
                                    <img
                                        src="./src/assets/ollama.png"
                                        width="20"
                                        height="20"
                                        alt="Ollama logo"
                                        class="invert-100"
                                    />
                                    <span
                                        class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap"
                                    >
                                        {model}
                                    </span>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                <div class="dropdown-wrapper">
                    <button
                        onclick={() => {
                            useThinking = !useThinking;
                            if (
                                useThinking === false &&
                                conversation.at(-1)?.isThinking
                            ) {
                                clearInterval(thinkingInterval);
                                thinkingInterval = null;
                                conversation.at(-1).isThinking = false;
                            }
                        }}
                        class="w-full text-center p-sm rounded-md"
                        class:bg-brand-primary={useThinking}
                        class:bg-black-200={!useThinking}
                    >
                        <Icon iconName="brain" width="20" />
                    </button>
                </div>
            </div>

            <div class="flex justify-between items-center space-x-sm">
                <input
                    type="text"
                    placeholder="Ask a follow-up"
                    class="bg-black-100 px-sm py-sm rounded-md w-full"
                    bind:value={message}
                    onkeypress={(e) => e.key === "Enter" && handleUserMessage()}
                />
                {#if isGenerating}
                    <Button intent="icon" handleClick={stopGenerating}>
                        <Icon iconName="close" width="20" />
                    </Button>
                {:else}
                    <Button intent="icon" handleClick={handleUserMessage}>
                        <Icon iconName="send" width="20" />
                    </Button>
                {/if}
            </div>
        </div>
    </section>
{/if}

<style>
    .dropdown-wrapper {
        position: relative;
    }
    .loader-icon {
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-top: 2px solid #fff;
        border-radius: 50%;
        width: 12px;
        height: 12px;
    }
    .loader-icon-lg {
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-top: 2px solid #fff;
        border-radius: 50%;
        width: 16px;
        height: 16px;
    }
    .animate-spin {
        animation: spin 1s linear infinite;
    }
    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>
