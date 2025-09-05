<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { showModelCreation } from "../../lib/stores/settings/settings";
    import { notifications } from "../../lib/stores/notifications/notification-store";

    let webModels = $state<string[]>([]);
    let pullingModels = $state<{ [key: string]: boolean }>({});


    async function getWebModels() {
        try {
            webModels = await invoke("get_web_models_cmd");
        } catch (e) {
            console.error(e);
        }
    }

    async function pullModel(modelName: string) {
        pullingModels = { ...pullingModels, [modelName]: true };
        notifications.add(`Downloading model: ${modelName}`, "info");
        
        try {
            await invoke("create_ollama_model_cmd", { modelName });
            notifications.add(`Download completed for model: ${modelName}`, "success");
        } catch(e) {
            console.error(e);
            notifications.add(`Download failed for model: ${modelName}`, "error");
        } finally {
            pullingModels = { ...pullingModels, [modelName]: false };
        }
    }

    onMount(async () => {
        await getWebModels();
    });
</script>

<div class="overflow-y-auto">
    <Button intent="icon" class="flex items-center gap-x-xs" onClick={() => $showModelCreation = false}>
        <Icon iconName="back" />
        <p>Back</p>
    </Button>
    {#if !webModels || webModels.length === 0}
        <div class="flex flex-col items-center justify-center p-xl text-center">
            <div class="cube-loader">
                <div class="cube-top"></div>
                <div class="cube-wrapper">
                    <span class="cube-span" style="--i:1;"></span>
                    <span class="cube-span" style="--i:2;"></span>
                    <span class="cube-span" style="--i:3;"></span>
                    <span class="cube-span" style="--i:4;"></span>
                </div>
            </div>
            <p class="mt-md text-lg text-black-300 animate-pulse">
                Fetching models from the web, please wait...
            </p>
        </div>
    {/if}
    {#if webModels && webModels.length > 0}
        {#each webModels as model}
            <ul class="px-md">
                <li class="px-0 py-sm flex flex-col gap-y-xs">
                    <div class="flex flex-col gap-y-xs flex-1">
                        <p class="text-lg font-bold">{model.model_name}</p>
                        <p class="text-sm text-black-300">
                            {model.description}
                        </p>
                        <div class="flex gap-x-sm mt-xs">
                            {#each model.capabilities as capability}
                                <p
                                    class="bg-utils-blue/20 text-utils-blue rounded-md py-1 px-1"
                                >
                                    {capability}
                                </p>
                            {/each}
                            {#each model.sizes as size}
                                <p
                                    class="bg-brand-primary/20 rounded-md px-xs py-xs text-brand-primary"
                                >
                                    {size}
                                </p>
                            {/each}
                        </div>
                        <div class="flex gap-x-sm">
                            <p class="flex gap-x-xs text-black-300">
                                <Icon iconName="download" width="20" />
                                {model.pulls} Pulls
                            </p>
                            <p class="flex gap-x-xs text-black-300">
                                <Icon iconName="clock" width="20" />
                                Updated {model.date}
                            </p>
                        </div>
                    </div>
                    <div>
                        {#if pullingModels[model.model_name]}
                            <Button intent="primary" disabled>
                                <span class="flex items-center gap-x-xs">
                                    <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    Pulling...
                                </span>
                            </Button>
                        {:else}
                            <Button intent="primary" onClick={() => pullModel(model.model_name)}>
                                Pull model
                            </Button>
                        {/if}
                    </div>
                </li>
            </ul>
        {/each}
    {/if}
</div>

<style>
    .cube-loader {
        position: relative;
        width: 70px;
        height: 70px;
        transform-style: preserve-3d;
        transform: rotateX(-30deg) rotateY(45deg);
        animation: rotate 2s linear infinite;
    }

    .cube-loader .cube-top {
        position: absolute;
        width: 70px;
        height: 70px;
        background: #111;
        transform: rotateX(90deg) translateZ(35px) translateX(35px);
    }

    .cube-loader .cube-wrapper {
        position: absolute;
        width: 70px;
        height: 70px;
        transform-style: preserve-3d;
    }

    .cube-loader .cube-wrapper .cube-span {
        position: absolute;
        width: 70px;
        height: 70px;
        background: #151515;
        border: 2px solid #333;
        box-sizing: border-box;
    }

    .cube-loader .cube-wrapper .cube-span:nth-child(1) {
        transform: translateZ(35px);
    }

    .cube-loader .cube-wrapper .cube-span:nth-child(2) {
        transform: rotateY(90deg) translateZ(35px);
    }

    .cube-loader .cube-wrapper .cube-span:nth-child(3) {
        transform: rotateY(180deg) translateZ(35px);
    }

    .cube-loader .cube-wrapper .cube-span:nth-child(4) {
        transform: rotateY(270deg) translateZ(35px);
    }

    @keyframes rotate {
        0% {
            transform: rotateX(-30deg) rotateY(45deg);
        }
        100% {
            transform: rotateX(-30deg) rotateY(405deg);
        }
    }

    .animate-pulse {
        animation: pulse-animation 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }

    @keyframes pulse-animation {
        0%, 100% {
            opacity: 1;
        }
        50% {
            opacity: .5;
        }
    }
</style>
