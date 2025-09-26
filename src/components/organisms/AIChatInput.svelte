<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, tick, onDestroy } from "svelte";
  import {
    aiIsLoading,
    aiMessages,
    chatContainer,
    selectedModel,
  } from "../../lib/stores/ai/ai-store";
  import { notifications } from "../../lib/stores/notifications/notification-store";
  import Button from "@components/atoms/Button.svelte";
  import Icon from "@components/atoms/Icon.svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { t } from "$lib/i18n";
  let input = "";

  let isGenerating: boolean = false;
  let mode: "chat" | "agentic" = "chat";
  let showModeDropdown: boolean = false;
  let textareaElement: HTMLTextAreaElement | null = null;
  let unlistenThinking: UnlistenFn;
  let unlistenChunk: UnlistenFn;
  let unlistenEnd: UnlistenFn;
  let unlistenError: UnlistenFn;

  async function handleSendMessage() {
    if (isGenerating) {
      stopGenerating();
      return;
    }

    if (!input.trim()) return;

    if (!$selectedModel) {
      notifications.add($t('ai.selectModel'), "error");
      return;
    }

    aiMessages.update((msgs) => [
      ...msgs,
      { sender: "user", text: input, thinking: "" },
    ]);
    const currentInput = input;
    input = "";
    aiIsLoading.set(true);
    isGenerating = true;

    let aiResponsePlaceholder = { sender: "ai", text: "", thinking: "" };
    aiMessages.update((msgs) => [...msgs, aiResponsePlaceholder]);

    await tick();
    scrollToBottom();

    let currentAiMessageIndex = $aiMessages.length - 1;

    unlistenThinking = await listen("ollama-chat-thinking", (event) => {
      const thinkingText = event.payload as string;
      aiMessages.update((msgs) => {
        msgs[currentAiMessageIndex].thinking += thinkingText;
        return [...msgs];
      });
      scrollToBottom();
    });

    unlistenChunk = await listen("ollama-chat-part", (event) => {
      const chunk = event.payload as string;
      aiMessages.update((msgs) => {
        msgs[currentAiMessageIndex].text += chunk;
        return [...msgs];
      });
      scrollToBottom();
    });

    unlistenEnd = await listen("ollama-chat-end", () => {
      cleanupListeners();
    });

    unlistenError = await listen("ollama-chat-error", (event) => {
      const error = event.payload as string;
      notifications.add(`Error: ${error}`, "error");
      aiMessages.update((msgs) => {
        msgs[currentAiMessageIndex].text =
          $t('ai.noResponse');
        msgs[currentAiMessageIndex].thinking = "";
        return [...msgs];
      });
      cleanupListeners();
    });

    try {
      await invoke("chat_with_ai_cmd", {
        prompt: currentInput,
        modelName: $selectedModel,
        useTools: false,
      });
    } catch (e) {
      notifications.add($t("ai.noMsg"), "error");
      aiMessages.update((msgs) => {
        msgs[$aiMessages.length - 1].text =
           $t('ai.noResponse');
        msgs[$aiMessages.length - 1].thinking = "";
        return [...msgs];
      });
      cleanupListeners();
    }
  }

  function cleanupListeners() {
    aiIsLoading.set(false);
    isGenerating = false;
    unlistenChunk();
    unlistenEnd();
    unlistenError();
    unlistenThinking();
    scrollToBottom();
  }

  function scrollToBottom() {
    if ($chatContainer) {
      $chatContainer.scrollTop = $chatContainer.scrollHeight;
    }
  }

  async function stopGenerating() {
    cleanupListeners();
    aiMessages.update((msgs) => {
      msgs[$aiMessages.length - 1].text = $t('ai.genStopped');
      msgs[$aiMessages.length - 1].thinking = "";
      return [...msgs];
    });
    await invoke("cancel_chat_stream_cmd");
  }

  function resizeTextarea() {
    if (textareaElement) {
      textareaElement.style.height = "auto";
      textareaElement.style.height = `${textareaElement.scrollHeight}px`;
    }
  }

  onMount(() => {
    scrollToBottom();
  });

  onDestroy(() => {
    if (unlistenThinking) unlistenThinking();
    if (unlistenChunk) unlistenChunk();
    if (unlistenEnd) unlistenEnd();
    if (unlistenError) unlistenError();
  });
</script>

<div class="w-full border border-black-200 rounded-md mb-md px-xs py-xs">
  <textarea
    bind:this={textareaElement}
    class="w-full px-sm py-xs rounded-md resize-none focus:border-none focus:outline-none resizable-textarea"
    placeholder={$t('ai.inputPlaceholder')}
    bind:value={input}
    oninput={resizeTextarea}
    onkeypress={(e) => {
      if (e.key === "Enter") {
        handleSendMessage();
        if (input.trim() === "" && isGenerating) {
          e.preventDefault();
        } else if ( input.trim() !== "") {
          e.preventDefault();
        }
        resizeTextarea();
      }
    }}
  ></textarea>
  <div class="flex justify-between items-center">
    <div class="flex space-x-xs items-center">
      <div class="dropdown-wrapper px-xs">
        <button
          onclick={() => (showModeDropdown = !showModeDropdown)}
          class="flex items-center text-white font-bold rounded-md cursor-pointer"
          title={mode === "chat" ? "Chat Mode" : "Agentic Mode"}
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
            role="list"
          >
            <button
              class="block w-full text-left px-sm py-xs hover:bg-black-200 rounded-md flex items-center space-x-sm cursor-pointer"
              onclick={() => {
                mode = "chat";
                showModeDropdown = false;
              }}
            >
              <Icon iconName="ai" width="20" />
              <span>{$t('ai.chatMode')}</span>
            </button>
            <button
              class="block w-full text-left px-sm py-xs hover:bg-black-200 rounded-md flex items-center space-x-sm cursor-pointer"
              onclick={() => {
                mode = "agentic";
                showModeDropdown = false;
              }}
            >
              <Icon iconName="stars" width="20" />
              <span>{$t('ai.agenticMode')}</span>
            </button>
          </div>
        {/if}
      </div>
      <Button intent="icon">
        <Icon iconName="internet" width="16" />
      </Button>
    </div>

    {#if isGenerating}
      <Button
        intent="iconStop"
        onClick={stopGenerating}
        class="bg-brand-primary-dark/30 rounded-md px-sm py-xs duration-200 transition"
      >
        <Icon iconName="stop" width="16" />
      </Button>
    {:else}
      <Button
        intent="icon"
        onClick={handleSendMessage}
        class="bg-black-100 rounded-md px-sm py-xs hover:bg-black-200 duration-200 transition"
      >
        <Icon iconName="send" width="16" />
      </Button>
    {/if}
  </div>
</div>

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
