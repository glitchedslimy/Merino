<script lang="ts">
    import { fade } from "svelte/transition";
    import { spacesStore } from "../../lib/stores/workspace/spaces-store";
    import Button from "@components/atoms/Button.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { loadSpaces } from "../../lib/actions/workspace/spaces";
    import { handleClickOutside } from "../../lib/useHooks/click-outside";
    import Icon from "@components/atoms/Icon.svelte";
    import { showModelCreation, showSpaceCreation } from "../../lib/stores/settings/settings";
    import { toasts } from "../../lib/stores/notifications/toast-store";
    import { isOllamaRunning } from "../../lib/stores/ai/ai-store";
    import AiNotAvailable from "./AINotAvailable.svelte";
    import { getOllamaModels } from "../../lib/api/tauri/get/ai-api-get";
    import { onMount } from "svelte";
    import ModelCreation from "./ModelCreation.svelte";

    let activeModelName = $state<string | null>(null);
    let parentRect = $state<DOMRect | null>(null);
    let models = $state<string[] | null>(null);

    async function getModels() {
        models = await getOllamaModels()
    }

    function toggleMenu(event: MouseEvent, modelName: string) {
        if (activeModelName === modelName) {
            activeModelName = null;
            parentRect = null;
        } else {
            activeModelName = modelName;
            parentRect =
                (event.currentTarget as HTMLElement)
                    .closest("div")
                    ?.getBoundingClientRect() ?? null;
        }
    }

    async function handleDelete(modelName: string) {
        try {
            await invoke("delete_ollama_model_cmd", { modelName });
            toasts.add(`Deleted space ${modelName}`, "success");
            await getModels()
        } catch (e) {
            console.error(e);
        }
        activeModelName = null;
        parentRect = null;
    }
    onMount(async () => {
        await getModels()
    })

    let showDropdown = $derived(
        activeModelName !== null && parentRect !== null,
    );
</script>

<div class="flex flex-col w-full">
    {#if !$isOllamaRunning}
        <AiNotAvailable />
    {:else}
        {#if $showModelCreation}
            <ModelCreation />
        {:else}
        <div class="flex flex-col items-center w-full overflow-y-auto">
            {#if models }
                <ul class="w-full rounded-lg bg-black-100">
                    {#each models as model}
                        <li>
                            <div
                                class="flex items-center justify-between p-4 cursor-pointer hover:bg-black-200
                     transition-colors duration-200 border-b border-black-200
                     last:border-b-0 rounded-md"
                            >
                                <span class="text-white font-medium truncate">
                                    {model.name}
                                </span>

                                <button
                                    onclick={(e) => toggleMenu(e, model.name)}
                                    class="relative z-10 py-1 px-1.5 rounded-full text-white hover:bg-black-300 transition-colors duration-200"
                                >
                                    <Icon iconName="dotsvertical" width="20" />
                                </button>
                            </div>
                        </li>
                    {/each}
                </ul>
            {:else}
                <p class="text-black-400 mb-sm">No spaces found.</p>
            {/if}
        </div>
        <div class="flex justify-end w-full items-end">
            <Button
                intent="primary"
                onClick={() => ($showModelCreation = !$showModelCreation)}
            >
                Pull a new model
            </Button>
        </div>
        {/if}
    {/if}
</div>

{#if showDropdown}
    <div
        use:handleClickOutside
        onclick_outside={() => {
            activeModelName = null;
            parentRect = null;
        }}
        transition:fade={{ duration: 150 }}
        class="absolute z-50 w-48 rounded-md shadow-lg
           bg-black-100 focus:outline-none"
        style="top: {parentRect.bottom - 150}px; right: {window.innerWidth -
            parentRect.right -
            192}px;"
    >
        <div class="py-1">
            <button
                onclick={() => handleDelete(activeModelName ?? "")}
                class="flex items-center w-full text-left px-4 py-2 text-sm text-utils-red hover:bg-black-200"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-4 w-4 mr-2"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="M3 6h18" />
                    <path
                        d="M19 6v14c0 1.105-.895 2-2 2H7c-1.105 0-2-.895-2-2V6"
                    />
                    <path d="M8 6V4c0-1.105.895-2 2-2h4c1.105 0 2 .895 2 2v2" />
                </svg>
                Delete
            </button>
        </div>
    </div>
{/if}
