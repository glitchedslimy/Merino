<script lang="ts">
    import { t } from "$lib/i18n";
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { open } from "@tauri-apps/plugin-shell";
</script>
<div class="flex flex-col items-center justify-center px-sm py-lg">
    <Icon iconName="ollama" width="200" height="100"/>
    <p class="px-sm py-lg">{$t('ai.ollama.title')}</p>
    <div class="flex flex-col gap-y-sm">
    <Button intent="primary" onClick={() => open("https://ollama.com/download")}>
        {$t('ai.ollama.getOllama')}
    </Button>
    <Button intent="gray">
        {$t('ai.ollama.externalModel')}
    </Button>
    </div>
</div>