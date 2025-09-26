<script lang="ts">
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import {
        activeTheme,
        primaryColor,
    } from "../../lib/stores/settings/settings";
    import { loadThemes } from "../../lib/actions/themes/load-themes";
    import { onMount } from "svelte";
    import { loadSettings } from "../../lib/actions/settings/load-settings";
    import { fade } from "svelte/transition";

    let themes: any[] | undefined = $state([]);

    // Create a new style element to inject custom themes
    const styleElement = document.createElement("style");
    styleElement.id = "dynamic-theme";
    document.head.appendChild(styleElement);

    // Function to update a setting on the backend
    async function updateSetting(key: string, value: string) {
        let parsedValue: any;

        try {
            parsedValue = JSON.parse(value); // try to parse value if it's JSON
        } catch {
            parsedValue = value; // otherwise just use it as string
        }

        const newSetting = JSON.stringify({ [key]: parsedValue });
        console.log("Setting", newSetting)
        try {
            await invoke("update_settings_cmd", { newSetting });
        } catch (e) {
            console.error(`Failed to save ${key}: `, e);
        }
    }

    // This function handles the theme selection from the dropdown
    function handleThemeSelect(themeName: string | null) {
        activeTheme.set(themeName);
    }

    // Debounce timers for each setting to prevent race conditions
    let primaryColorTimeout: number | null = null;
    let activeThemeTimeout: number | null = null;

    // Load both settings and themes before the component renders
    onMount(async () => {
        await loadSettings();
        themes = await loadThemes();
    });

    // Effect to apply changes and save them
    $effect(() => {
        // Save primary color and apply the CSS variable
        if ($primaryColor) {
            if (primaryColorTimeout) clearTimeout(primaryColorTimeout);
            primaryColorTimeout = window.setTimeout(() => {
                updateSetting("primaryColor", $primaryColor ?? "");
            }, 500);
        }
        document.documentElement.style.setProperty(
            "--color-brand-primary",
            $primaryColor,
        );

        // Save active theme and apply the data-theme attribute
        if ($activeTheme) {
            if (activeThemeTimeout) clearTimeout(activeThemeTimeout);
            activeThemeTimeout = window.setTimeout(() => {
                updateSetting("theme", $activeTheme ?? "");
            }, 500);
            document.documentElement.setAttribute("data-theme", $activeTheme);

            // Fetch theme CSS content from backend and inject
            (async () => {
                if ($activeTheme === "default") {
                    styleElement.innerHTML = ""; // Clear custom theme styles
                } else {
                    try {
                        const themeCss: string = await invoke(
                            "get_theme_content_cmd",
                            { themeName: $activeTheme },
                        );
                        styleElement.innerHTML = themeCss;
                    } catch (e) {
                        console.error("Failed to load theme content:", e);
                        styleElement.innerHTML = "";
                    }
                }
            })();
        }
    });
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
            <Dropdown
                options={themes}
                onSelect={handleThemeSelect}
                selectedOption={$activeTheme ?? ""}
            />
        </div>
    </div>
    <div class="flex justify-between">
        <div>
            <p>Accent color</p>
            <p class="text-sm text-black-400 mb-2">
                Changes the accent color of the theme used.
            </p>
        </div>
        <div>
            <div class="space-y-4">
                <div
                    class="relative w-6 h-6 rounded-full border border-black overflow-hidden cursor-pointer"
                >
                    <input
                        type="color"
                        id="colorSelect"
                        class="absolute inset-0 w-full h-full cursor-pointer z-10 color-input"
                        bind:value={$primaryColor}
                    />
                    <div
                        class="absolute inset-0 z-0"
                        style="background-color: {$primaryColor};"
                    ></div>
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
