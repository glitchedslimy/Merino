<script>
    import Button from "@components/atoms/Button.svelte";
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import { check } from "@tauri-apps/plugin-updater";
    import { app } from "@tauri-apps/api";
    import { onMount } from "svelte";
    const dropdownOptions = [{ name: "Spanish" }, { name: "English" }];

    let appVersion = "v0.0.0";
    let updateMessage = "Checking...";
    let isUpdateAvailable = false;

    app.getVersion().then((v) => (appVersion = `v${v}`));

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
        updateMessage = "Checking for updates...";
        const update = await check();
        console.log(await check().then((t) => t?.body));
        try {
            if (update) {
                isUpdateAvailable = true;
                updateMessage = `Update available: v${update.version}`;
            } else {
                isUpdateAvailable = false;
                updateMessage = "Your app is up to date!";
            }
        } catch (e) {
            isUpdateAvailable = false;
            updateMessage = "Failed to check updates.";
            console.error(e);
        }
    }

    onMount(async () => {
        await runUpdater();
    });
</script>

<div class="w-full">
    <p class="mb-sm">Application</p>
    <div class="flex items-center justify-between mb-md">
        <div>
            <p class="text-sm">Application Version: {appVersion}</p>
            <p class="text-sm text-black-300">{updateMessage}</p>
            <a
                href="https://github.com/glitchedslimy/Merino/blob/main/CHANGELOG.md"
                class="text-sm text-brand-primary underline"
                target="_blank">Read the change logs</a
            >
        </div>
        <div>
            {#if isUpdateAvailable}
                <Button intent="primary" onClick={installUpdate}>
                    Install update
                </Button>
            {:else}
                <Button intent="primary" onClick={runUpdater}>
                    Search for updates
                </Button>
            {/if}
        </div>
    </div>
    <div class="flex items-center justify-between">
        <div>
            <p class="text-sm">Language</p>
            <p class="text-sm text-black-300">
                Change the language to display. <b
                    >(Not functional right now.)</b
                >
            </p>
        </div>
        <div>
            <Dropdown options={dropdownOptions} selectedOption={"English"} />
        </div>
    </div>
</div>
