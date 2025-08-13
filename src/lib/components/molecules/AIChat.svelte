<script lang="ts">
    // 💡 Essential Svelte lifecycle functions for component management.
    // `onMount` runs when the component is added to the DOM.
    // `onDestroy` runs when the component is removed from the DOM.
    import { onMount, onDestroy } from "svelte";

    // 💡 Tauri API for listening to events from the Rust backend.
    // UnlistenFn is a function returned by the listener to stop listening.
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";

    // 💡 Custom utility functions from your project's backend.
    // These functions likely call a Tauri command to communicate with Ollama.
    import {
        sendToChat,
        getOllamaModels,
        stopStreaming,
    } from "../../utils/ai-chat";

    // 💡 Custom Svelte components for UI elements.
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import Markdown from "@components/molecules/Markdown.svelte";
    import Tooltip from "@components/atoms/Tooltip.svelte";

    // 💡 Svelte store to manage the chat window's visibility across the application.
    import { openAiChat } from "@stores/ai-store";

    // 💡 Svelte motion transitions for smooth UI animations.
    import { slide } from "svelte/transition";

    // 💡 Custom action for ellipsis detection to show a tooltip.
    import { ellipsisTooltip } from "../../utils/ellipsis-detection";
    import { getModelLogo } from "../../utils/get-model-logos";

    // 💡 `style` is a prop passed into this component, likely for positioning.
    let { style } = $props();

    // 💡 Function to close the chat window by updating the Svelte store.
    function closeChat() {
        openAiChat.set(false);
    }

    // === Tooltip State and Functions ===
    let timeoutId: number | null = null;
    const DELAY_MS = 500;
    let showTooltip = $state(false);
    let tooltipText = $state("");
    let tooltipX = $state(0);
    let tooltipY = $state(0);

    function handleShowTooltip(text: string, x: number, y: number) {
        if (timeoutId) {
            clearTimeout(timeoutId);
        }
        timeoutId = setTimeout(() => {
            showTooltip = true;
            tooltipText = text;
            tooltipX = x;
            tooltipY = y - 40; // Position tooltip slightly above the element
        }, DELAY_MS);
    }

    function handleHideTooltip() {
        if (timeoutId) {
            clearTimeout(timeoutId);
            timeoutId = null;
        }
        showTooltip = false;
        tooltipText = "";
    }
    // ===================================

    // === Chat State Variables ===
    let message: string = $state("");
    let conversation: {
        text?: string;
        sender: "user" | "ai" | "thinking-steps" | "loading-ai" | "system";
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
    let isStoppedByUser: boolean = $state(false);
    let cleanupTimeoutId: number | null = null; // Timeout for generation cleanup

    let models: { name: string; capabilities: string[] }[] = $state([]);
    let selectedModel: string = $state("llama3.1");
    let showModelDropdown: boolean = $state(false);
    let isFetchingModels: boolean = $state(true);

    let thinkingInterval: number | null = null;
    let isLoadingAiResponse: boolean = $state(false);
    let liveThinkingDuration = $state(0);
    let startTime = $state(0);

    let unlisten: UnlistenFn;

    function getSelectedModelCapabilities() {
        const currentModel = models.find((m) => m.name === selectedModel);
        return currentModel?.capabilities || [];
    }

    // 💡 Sets up the event listeners for chat chunks and end events from the Tauri backend.
    async function setupListeners() {
        // Listen for streaming chunks of the AI's response
        const unlistenChunk = await listen<any>(
            "ollama-chat-chunk",
            (event) => {
                const chunk = event.payload;

                // If the user has stopped the generation, discard any late-arriving chunks.
                if (isStoppedByUser) {
                    console.warn(
                        "Generation was stopped. Discarding late chunk:",
                        chunk,
                    );
                    return;
                }

                // --- Start of modified logic for 'thinking' chunks ---
                const lastMessage = conversation[conversation.length - 1];

                // Handle thinking steps if the model supports it.
                if (
                    chunk.message?.thinking &&
                    getSelectedModelCapabilities().includes("thinking")
                ) {
                    isLoadingAiResponse = false;

                    // If the last message is already a 'thinking-steps' message, append to it.
                    if (
                        lastMessage &&
                        lastMessage.sender === "thinking-steps" &&
                        lastMessage.isThinking
                    ) {
                        lastMessage.steps.push(chunk.message.thinking);
                    } else {
                        // Otherwise, this is a new 'thinking-steps' block, so add a new one.
                        conversation.push({
                            sender: "thinking-steps",
                            steps: [chunk.message.thinking],
                            duration: 0,
                            expanded: false,
                            isThinking: true,
                            liveDuration: 0,
                        });
                    }
                    conversation = conversation; // Trigger a Svelte update
                }
                // --- End of modified logic for 'thinking' chunks ---

                // Handle the main AI response content.
                else if (chunk.message?.content) {
                    isLoadingAiResponse = false;
                    const lastMessage = conversation[conversation.length - 1];
                    // Append to the last message if it's also from the AI.
                    if (lastMessage && lastMessage.sender === "ai") {
                        lastMessage.text += chunk.message.content;
                    } else {
                        // Otherwise, start a new AI message.
                        conversation.push({
                            text: chunk.message.content,
                            sender: "ai",
                        });
                    }
                    conversation = conversation; // Trigger a Svelte update
                    cleanupAfterThinking(); // Stop the thinking duration timer
                }
            },
        );

        // Listen for the end of the chat stream
        const unlistenEnd = await listen("ollama-chat-end", () => {
            // Clear the fallback cleanup timeout if the official end event is received.
            if (cleanupTimeoutId) {
                clearTimeout(cleanupTimeoutId);
                cleanupTimeoutId = null;
            }

            console.log("ollama-chat-end event received.");

            // Update the history of messages for the next conversation round.
            messagesForOllama = conversation
                .filter((msg) => msg.sender === "user" || msg.sender === "ai")
                .map((msg) => ({
                    role: msg.sender === "user" ? "user" : "assistant",
                    content: msg.text,
                }));

            cleanupAfterGeneration();
            console.log("Final conversation state:", conversation);
        });

        // --- Start of new listener for cancellation event ---
        const unlistenCancel = await listen("ollama-chat-cancel", () => {
            console.log("ollama-chat-cancel event received.");

            // Set the message for cancellation
            conversation.push({
                text: "Generation was cancelled by the user.",
                sender: "system",
            });

            cleanupAfterGeneration(); // Perform the necessary cleanup
        });
        // --- End of new listener ---

        unlisten = () => {
            unlistenChunk();
            unlistenEnd();
            unlistenCancel(); // Clean up the new listener
        };
    }

    // 💡 Lifecycle hook: runs when the component is mounted to the DOM.
    onMount(async () => {
        console.log("AIChat component mounted. Fetching models...");
        isFetchingModels = true;

        // Fetch available Ollama models.
        const fetchedModels = await getOllamaModels();
        if (fetchedModels.length > 0) {
            models = fetchedModels;
            const defaultModel = models.find((m) => m.name === "llama3.1");
            selectedModel = defaultModel ? defaultModel.name : models[0].name;
        }
        isFetchingModels = false;

        // Set up the event listeners for communication with the backend.
        await setupListeners();
    });

    // 💡 Lifecycle hook: runs when the component is unmounted.
    onDestroy(() => {
        console.log("AIChat component destroyed. Unlistening from events.");
        if (unlisten) {
            unlisten(); // Clean up the Tauri event listener
        }
    });

    // 💡 Handles sending a message from the user.
    async function handleUserMessage() {
        // Prevent sending if the message is empty or a generation is already in progress.
        if (!message.trim() || isFetchingModels || isGenerating) return;

        isGenerating = true;
        isStoppedByUser = false;
        isLoadingAiResponse = true;

        const userMessage = message;
        conversation.push({ text: userMessage, sender: "user" }); // Add user message to conversation
        messagesForOllama.push({ role: "user", content: userMessage }); // Add to Ollama history

        message = ""; // Clear the input field

        await processChatResponse();
    }

    // 💡 Cleans up all state variables after a generation is fully complete.
    function cleanupAfterGeneration() {
        console.log("Cleaning up after generation.");
        isGenerating = false;
        isLoadingAiResponse = false;
        isStoppedByUser = false;
        cleanupAfterThinking(); // Also stop the thinking timer
    }

    // 💡 Stops the thinking timer once the final response starts arriving.
    function cleanupAfterThinking() {
        console.log("Cleaning up after thinking.");
        if (thinkingInterval) {
            clearInterval(thinkingInterval);
            thinkingInterval = null;
        }
        const thinkingMessage = conversation.find(
            (msg) => msg.sender === "thinking-steps" && msg.isThinking,
        );
        if (thinkingMessage) {
            thinkingMessage.isThinking = false;
            thinkingMessage.duration = liveThinkingDuration;
            conversation = conversation; // Trigger a Svelte update
        }
        liveThinkingDuration = 0;
    }

    // 💡 Starts the process of getting the AI's response.
    async function processChatResponse() {
        console.log("Processing chat response...");
        const useThinking = getSelectedModelCapabilities().includes("thinking");

        // If the model supports "thinking", add a new thinking message to the conversation.
        if (useThinking) {
            conversation.push({
                sender: "thinking-steps",
                steps: [],
                duration: 0,
                expanded: false,
                isThinking: true,
                liveDuration: 0,
            });
            conversation = conversation; // Trigger a Svelte update
            console.log("Thinking message added:", conversation);

            // Start the timer for "thinking" duration.
            startTime = Date.now();
            thinkingInterval = setInterval(() => {
                liveThinkingDuration = Date.now() - startTime;
            }, 100);
        }

        try {
            // Call the Tauri command to start the chat stream.
            await sendToChat(
                messagesForOllama,
                useThinking,
                selectedModel,
                useThinking,
            );
        } catch (error) {
            console.error("Failed to start chat stream:", error);
            cleanupAfterGeneration();
            return;
        }
    }

    // 💡 Function to stop the ongoing AI generation.
    async function stopGenerating() {
        isStoppedByUser = true;
        await stopStreaming(); // Call the Tauri command to stop the stream.
        // The `ollama-chat-cancel` event from the backend will now handle the rest of the cleanup.
    }

    // === Textarea Auto-Resize Logic ===
    let textareaElement: HTMLTextAreaElement | null = $state(null);

    function resizeTextarea() {
        if (textareaElement) {
            // Reset the height to 'auto' to get the correct scrollHeight.
            textareaElement.style.height = "auto";
            // Set the new height, capped by the CSS max-height.
            textareaElement.style.height = `${textareaElement.scrollHeight}px`;
        }
    }

    // 💡 Reactive effect that resizes the textarea whenever the message content changes.
    $effect(() => {
        message.length;
        resizeTextarea();
    });
</script>

{#if $openAiChat}
    <section
        class="bg-black rounded-md w-72 px-md pt-md flex flex-col justify-between"
        transition:slide={{ duration: 200, axis: "x" }}
        {style}
    >
        <div class="flex justify-between items-center mb-sm">
            <Button intent="icon" handleClick={closeChat}>
                <Icon iconName="chevron-pipe-right" width="20" />
            </Button>

            <div class="relative">
                <button
                    onclick={() => (showModelDropdown = !showModelDropdown)}
                    class="flex items-center text-white text-sm font-medium rounded-md space-x-xs justify-between cursor-pointer max-w-2xl"
                >
                    <img
                        src={getModelLogo(selectedModel)}
                        width="16"
                        height="20"
                        alt="Model logo"
                    />
                    <span
                        class="overflow-hidden text-ellipsis whitespace-nowrap"
                        use:ellipsisTooltip={{
                            onHide: handleHideTooltip,
                            onShow: handleShowTooltip,
                        }}
                    >
                        {isFetchingModels ? "Loading..." : selectedModel.split(":")[0]}
                    </span>
                    <div
                        class={`transition-all duration-200 ${showModelDropdown ? "rotate-180" : "rotate-0"}`}
                    >
                        <Icon iconName="chevron-down" width="16" />
                    </div>
                </button>
                {#if showModelDropdown && models.length > 0}
                    <div
                        class="absolute z-10 bg-black-100 rounded-md flex flex-col space-y-sm text-sm top-full mt-sm max-w-[150px]"
                        onmouseleave={() => (showModelDropdown = false)}
                    >
                        {#each models as model}
                            <div
                                class="flex items-center rounded-md hover:bg-black-200 space-x-sm px-sm py-sm cursor-pointer"
                                onclick={() => {
                                    selectedModel = model.name;
                                    showModelDropdown = false;
                                }}
                            >
                                <img
                                    src={getModelLogo(model.name)}
                                    width="16"
                                    height="20"
                                    alt="Ollama logo"
                                />
                                <span
                                    class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap"
                                    use:ellipsisTooltip={{
                                        onHide: handleHideTooltip,
                                        onShow: handleShowTooltip,
                                    }}
                                >
                                    {model.name}
                                </span>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
            <div>
                <Icon iconName="WoolyStroke" width="20" />
            </div>
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
                                        <span
                                            >{(
                                                liveThinkingDuration / 1000
                                            ).toFixed(2)}s - Thinking</span
                                        >
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
                {:else if chatMessage.sender === "system"}
                    <div class="mb-sm text-center">
                        <span class="text-sm text-gray-400">
                            {chatMessage.text}
                        </span>
                    </div>
                {/if}
            {/each}

            {#if isLoadingAiResponse}
                <div class="mb-sm">
                    <div class="inline-block p-sm rounded-lg bg-black-100">
                        <div class="flex items-center space-x-xs text-white">
                            <span class="animate-spin loader-icon-lg"></span>
                            <span>Loading response...</span>
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        <div
            class="w-full border border-black-200 rounded-md mb-md px-xs py-xs"
        >
            <textarea
                bind:this={textareaElement}
                class="w-full px-sm py-xs rounded-md resize-none focus:border-none focus:outline-none resizable-textarea"
                placeholder="Ask a follow-up"
                bind:value={message}
                onkeypress={(e) => e.key === "Enter" && handleUserMessage() && e.preventDefault()}
            ></textarea>
            <div class="flex justify-between items-center">
                <div class="flex space-x-xs items-center">
                    <div class="dropdown-wrapper px-xs">
                        <button
                            onclick={() =>
                                (showModeDropdown = !showModeDropdown)}
                            class="flex items-center text-white font-bold rounded-md cursor-pointer"
                            title={mode === "chat"
                                ? "Chat Mode"
                                : "Agentic Mode"}
                        >
                            {#if mode === "agentic"}
                                <Icon iconName="stars" width="16" />
                            {:else}
                                <Icon iconName="ai" width="16" />
                            {/if}
                            <div
                                class={`transition-all duration-200 ${showModeDropdown ? "rotate-180" : "rotate-0"}`}
                            >
                                <Icon iconName="chevron-down" width="16" />
                            </div>
                        </button>
                        {#if showModeDropdown}
                            <div
                                class="absolute z-10 bottom-full bg-black-100 rounded-md"
                                onmouseleave={() => (showModeDropdown = false)}
                            >
                                <button
                                    class="block w-full text-left px-sm py-xs hover:bg-black-200 rounded-md flex items-center space-x-sm cursor-pointer"
                                    onclick={() => {
                                        mode = "chat";
                                        showModeDropdown = false;
                                    }}
                                >
                                    <Icon iconName="ai" width="20" />
                                    <span>Chat Mode</span>
                                </button>
                                <button
                                    class="block w-full text-left px-sm py-xs hover:bg-black-200 rounded-md flex items-center space-x-sm cursor-pointer"
                                    onclick={() => {
                                        mode = "agentic";
                                        showModeDropdown = false;
                                    }}
                                >
                                    <Icon iconName="stars" width="20" />
                                    <span>Agentic Mode</span>
                                </button>
                            </div>
                        {/if}
                    </div>
                    <Button intent="icon">
                        <Icon iconName="internet" width="16"/>
                    </Button>
                </div>

                {#if isGenerating}
                    <Button
                        intent="iconStop"
                        handleClick={stopGenerating}
                        class="bg-brand-primary-dark/30 rounded-md px-sm py-xs duration-200 transition"
                    >
                        <Icon iconName="stop" width="16" />
                    </Button>
                {:else}
                    <Button
                        intent="icon"
                        handleClick={handleUserMessage}
                        class="bg-black-100 rounded-md px-sm py-xs hover:bg-black-200 duration-200 transition"
                    >
                        <Icon iconName="send" width="16" />
                    </Button>
                {/if}
            </div>
        </div>
    </section>
{/if}

{#if showTooltip}
    <Tooltip text={tooltipText} x={tooltipX} y={tooltipY} />
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
    .resizable-textarea {
        resize: none;
        overflow-y: auto;
        min-height: 2.5rem;
        max-height: 7.5rem;
    }
</style>
