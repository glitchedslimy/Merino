<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { showSpaceCreation } from "../../lib/stores/settings/settings";
    import { invoke } from "@tauri-apps/api/core";
    import { loadSpaces } from "../../lib/actions/workspace/spaces";
    import { toasts } from "../../lib/stores/notifications/toast-store";

    let spaceNamevalue = $state<string>("");

    async function createSpace(spaceName: string) {
        try {
            await invoke("create_space_cmd", {spaceName})
            await loadSpaces()
            $showSpaceCreation = !$showSpaceCreation
            toasts.add(`Created ${spaceName} space!`, "success")
        } catch(e) {
            console.error(e);
        }
    }
</script>

<div>
    <div class="flex items-center justify-between mb-lg">
        <Button
            intent="icon"
            onClick={() => ($showSpaceCreation = !$showSpaceCreation)}
        >
            <Icon iconName="back" />
        </Button>
        <p class="font-bold">Space creation</p>
        <Icon iconName="WoolyStroke" width="20" />
    </div>
    <div class="flex flex-col gap-y-sm">
        <label for="space">Space name</label>
        <input
        type="text"
        class="w-full px-2 py-2 bg-black-100 border border-black-200 rounded-md mb-xs"
        placeholder="My epic namespace"
        bind:value={spaceNamevalue}
    />
    <Button
        intent="primary"
        onClick={() => createSpace(spaceNamevalue)}
      >
        Create
      </Button>
    </div>
</div>
