<script>
    import Button from "@components/atoms/Button.svelte";
    import { createNote } from "../../lib/actions/workspace/notes";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import woolyLogo from "../../assets/icons/WoolyStrokeClear.svg"
    import { showCommandPalette } from "../../lib/stores/commandpalette/commandpalette";
    import { t } from "$lib/i18n";
</script>

<div
    class="bg-black h-screen rounded-md mt-xs flex justify-center items-center flex-col space-y-md"
>
    <div class="-mt-24 flex flex-col items-center justify-center space-y-md">
        <img src={woolyLogo}
            alt="Wooly Stroke Logo"
            width="250"
        />
        <p class="text-black-400 text-2xl font-semibold">
            {$t('editor.noNotesOpen')}
        </p>
        <div class="flex flex-col space-y-sm items-center">
            <Button intent="link" onClick={() => createNote($activeSpace ?? '', null)}>{$t('editor.quickAccess.createNote')}</Button>
            <Button intent="link" onClick={() => $showCommandPalette = !$showCommandPalette}>{$t('editor.quickAccess.searchNote')}</Button>
        </div>
    </div>
</div>
