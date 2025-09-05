<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let webModels = $state<string[]>([]);
    async function getWebModels() {
        try {
            webModels = await invoke("get_web_models_cmd");
            console.log("Web models", webModels)
        } catch(e) {
            console.error(e)
        }
    }

    onMount(async () => {
        await getWebModels();
    })
</script>

<div>
    Model creation
</div>