<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { open } from "@tauri-apps/plugin-shell";
</script>
<div class="flex flex-col items-center justify-center px-sm py-lg">
    <Icon iconName="ollama" width="200" height="100"/>
    <p class="px-sm py-lg">Install or run Ollama to get the AI features.</p>
    <div class="flex flex-col gap-y-sm">
    <Button intent="primary" onClick={() => open("https://ollama.com/download")}>
        Get Ollama
    </Button>
    <Button intent="gray">
        I want to use an external model.
    </Button>
    </div>
</div>