<script lang="ts">
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { primaryColor } from "../../lib/stores/settings/settings";
    import { onMount } from "svelte";
    import { loadThemes } from "../../lib/actions/themes/load-themes";

    let themes = $state([]);

    const dropdownOptions = {
        theme1: "theme1",
        theme2: "theme1",
        theme3: "theme1",
    };

    async function updateSetting(key: string, value: string) {
        const newSetting = JSON.stringify({ [key]: value});
        try {
            await invoke("update_settings_cmd", { newSetting })
        } catch(e) {
            console.error(`Failed to saved ${key}: `, e);
        }
    }

    $effect(() => {
        let debounceTimeout: number | null = null;
        if ($primaryColor) {
            if(debounceTimeout) clearTimeout(debounceTimeout);
            debounceTimeout = window.setTimeout(() => {
                updateSetting("primaryColor", $primaryColor ?? "");
            }, 500);
        }
        document.documentElement.style.setProperty('--color-brand-primary', $primaryColor);
    });
    onMount(async () => {
        let themes = await loadThemes();
        console.log(themes)
    })
</script>

<div class="w-full flex flex-col gap-y-lg justify-start">
    <div class="flex justify-between">
        <div>
            <p>Selected theme</p>
            <p class="text-sm text-black-400 mb-2">
                The base theme selected on Merino.
            </p>
        </div>
        <div>
            <Dropdown options={dropdownOptions} selectedOption={"theme1"} />
        </div>
    </div>
    <div class="flex justify-between">
        <div>
            <p>Accent color</p>
            <p class="text-sm text-black-400 mb-2">
                <b>This only works with the default theme.</b> Changes the accent
                color of the theme used.
            </p>
        </div>
        <div>
            <div class="space-y-4">
                <div class="relative w-6 h-6 rounded-full border border-black overflow-hidden cursor-pointer">
                    <input
                        type="color"
                        id="colorSelect"
                        class="absolute inset-0 w-full h-full cursor-pointer z-10 color-input"
                        bind:value={$primaryColor}
                    />
                    <div class="absolute inset-0 z-0" style="background-color: {$primaryColor};"></div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    /* This customizes the color picker input. 
        It removes the default browser appearance to make it transparent, 
        so we can place our own styled div underneath it.
    */
    .color-input {
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        background-color: transparent;
        border: none;
    }
    .color-input::-webkit-color-swatch-wrapper {
        padding: 0;
    }
    .color-input::-webkit-color-swatch {
        border: none;
    }
</style>
