<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import ThemesSettings from "./ThemesSettings.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { openSettings } from "../../lib/stores/settings/settings";
    import AboutSettings from "./AboutSettings.svelte";
    import SpaceSettings from "./SpaceSettings.svelte";
    import { onMount } from "svelte";
    import Shortcuts from "./Shortcuts.svelte";
    import AiSettings from "./AISettings.svelte";
    import { t } from "$lib/i18n";

    let selectedOption = $state("about");
    let element: HTMLElement | null = null
    function handleEscKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") { 
            e.preventDefault(); 
            $openSettings = false;
        }
    }
    onMount(() => {
        element?.addEventListener("keydown", handleEscKeydown);
    })
</script>

<div class="bg-black-100 flex w-full mx-[15%] my-xl relative" bind:this={element}>
    <div class="bg-black-200/20 px-lg py-md flex flex-col gap-y-sm">
        <p class="text-black-300 text-sm mb-xs">{$t("settings.options")}</p>
        <Button
            intent="settings"
            onClick={() => (selectedOption = "about")}
            class={`${selectedOption === "about" ? "bg-brand-primary" : ""}`}
        >
            {$t("settings.about.title")}
        </Button>
        <Button
            intent="settings"
            onClick={() => (selectedOption = "spaces")}
            class={`${selectedOption === "spaces" ? "bg-brand-primary" : ""}`}
        >
            {$t("settings.spaces.title")}
        </Button>
        <Button
            intent="settings"
            onClick={() => (selectedOption = "appearance")}
            class={`${selectedOption === "appearance" ? "bg-brand-primary" : ""}`}
        >
            {$t("settings.appearance.title")}
        </Button>
        <Button
            intent="settings"
            onClick={() => (selectedOption = "shortcuts")}
            class={`${selectedOption === "shortcuts" ? "bg-brand-primary" : ""}`}
        >
            {$t("settings.shortcuts.title")}
        </Button>
        <Button
            intent="settings"
            onClick={() => (selectedOption = "ai")}
            class={`${selectedOption === "ai" ? "bg-brand-primary" : ""}`}
        >
            {$t("settings.ai.title")}
        </Button>
    </div>
    <div class="flex justify-start lg:mx-xl mx-sm my-xl w-full">
        {#if selectedOption === "appearance"}
            <ThemesSettings />
        {/if}
        {#if selectedOption === "about"}
            <AboutSettings />
        {/if}
        {#if selectedOption === "spaces"}
            <SpaceSettings />
        {/if}
        {#if selectedOption === "shortcuts"}
            <Shortcuts />
        {/if}
         {#if selectedOption === "ai"}
            <AiSettings />
        {/if}
    </div>
    <Button
        intent="icon"
        class="absolute right-sm top-sm"
        onClick={() => ($openSettings = !$openSettings)}
    >
        <Icon iconName="close" />
    </Button>
</div>
