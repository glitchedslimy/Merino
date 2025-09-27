<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import { check } from "@tauri-apps/plugin-updater";
    import { app } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { locale, t } from "$lib/i18n";
    import { derived } from "svelte/store";
    import { invoke } from "@tauri-apps/api/core";
    import { loadSettings } from "$lib/actions/settings/load-settings";
    const dropdownOptions = [
        { name: "Español", code: "es" },
        { name: "English", code: "en" },
        { name: "日本語", code: 'jp'}
    ];

    let appVersion = $state("v0.0.0");
    let updateMessage = $state($t("settings.about.application.checking"));
    let isUpdateAvailable = $state(false);

    const selectedOption = derived(locale, ($locale) => {
        switch($locale) {
            case "es": return 'Español';
            case "en": return "English";
            case 'jp': return '日本語';
            default: return "English";
        }
    });

    app.getVersion().then((v) => (appVersion = `v${v}`));

    async function updateSetting(key: string, value: string) {
        let parsedValue: any;

        try {
            parsedValue = JSON.parse(value); // try to parse value if it's JSON
        } catch {
            parsedValue = value; // otherwise just use it as string
        }

        const newSetting = JSON.stringify({ [key]: parsedValue });
        try {
            await invoke("update_settings_cmd", { newSetting });
        } catch (e) {
            console.error(`Failed to save ${key}: `, e);
        }
    }

    async function installUpdate() {
        try {
            const update = await check();
            if (update) {
                await update.downloadAndInstall();
                console.log("Update installed successfully");
            } else {
                console.log("No update available");
            }
        } catch (err) {
            console.error("Failed to install update:", err);
        }
    }

    async function runUpdater() {
        updateMessage = $t("settings.about.application.checkingUpdates");
        const update = await check();
        console.log(await check().then((t) => t?.body));
        try {
            if (update) {
                isUpdateAvailable = true;
                updateMessage = `${$t("settings.about.application.updateFound")} v${update.version}`;
            } else {
                isUpdateAvailable = false;
                updateMessage = $t("settings.about.application.upToDate");
            }
        } catch (e) {
            isUpdateAvailable = false;
            updateMessage = $t("settings.about.application.checkFail");
            console.error(e);
        }
    }

    function handleLanguageSelect(optionName: string) {
        const selected = dropdownOptions.find((o) => o.name === optionName);
        if (selected) locale.set(selected.code as "en" | "es");
        updateSetting("locale", selected!.code)
    }

    onMount(async () => {
        await loadSettings();
        await runUpdater();
    });
</script>

<div class="w-full">
    <p class="mb-sm">{$t("settings.about.application.title")}</p>
    <div class="flex items-center justify-between mb-md">
        <div>
            <p class="text-sm">{$t("settings.about.application.version")} {appVersion}</p>
            <p class="text-sm text-black-300">{updateMessage}</p>
            <a
                href="https://github.com/glitchedslimy/Merino/blob/main/CHANGELOG.md"
                class="text-sm text-brand-primary underline"
                target="_blank">{$t("settings.about.application.changelogs")}</a
            >
        </div>
        <div>
            {#if isUpdateAvailable}
                <Button intent="primary" onClick={installUpdate}>
                    {$t("settings.about.application.install")}
                </Button>
            {:else}
                <Button intent="primary" onClick={runUpdater}>
                   {$t("settings.about.application.search")}
                </Button>
            {/if}
        </div>
    </div>
    <div class="flex items-center justify-between">
        <div>
            <p class="text-sm">{$t("settings.about.language.title")}</p>
            <p class="text-sm text-black-300">
                {$t("settings.about.language.changeLang")}
            </p>
        </div>
        <div>
            <Dropdown
                options={dropdownOptions}
                selectedOption={$selectedOption}
                onSelect={handleLanguageSelect}
            />
        </div>
    </div>
</div>
