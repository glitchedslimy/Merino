<script>
    import Button from "@components/atoms/Button.svelte";
    import ThemesSettings from "./ThemesSettings.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { openSettings } from "../../lib/stores/settings/settings";

    let selectedOption = $state("about");
</script>
<div class="bg-black-100 flex w-full mx-[15%] my-xl relative">
    <div class="bg-black-200/20 px-lg py-md flex flex-col gap-y-sm">
        <p class="text-black-300 text-sm mb-xs">Options</p>
        <Button intent="settings" onClick={() => selectedOption = "about"} class={`${selectedOption === "about" ? "bg-brand-primary": ""}`}>
            About
        </Button>
        <Button intent="settings" onClick={() => selectedOption = "spaces"} class={`${selectedOption === "spaces" ? "bg-brand-primary": ""}`}>
            Spaces
        </Button>
        <Button intent="settings" onClick={() => selectedOption = "theming"} class={`${selectedOption === "theming" ? "bg-brand-primary": ""}`}>
            Theming
        </Button>
    </div>
    <div class="flex justify-start lg:mx-xl mx-sm my-xl w-full">
        {#if selectedOption === "theming"}
        <ThemesSettings />
        {/if}
    </div>
    <Button intent="icon" class="absolute right-sm top-sm" onClick={() => $openSettings = !$openSettings}>
        <Icon iconName="close" />
    </Button>
</div>