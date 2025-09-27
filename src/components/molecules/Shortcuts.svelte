<script>
    import { t } from "$lib/i18n";

</script>
<div class="w-full flex flex-col gap-y-sm">
    <p class="">{$t("settings.shortcuts.disclaimer")}</p>
    <div class="flex justify-between">
        <p>{$t("settings.shortcuts.shortcut.openSettings")}</p>
        <div class="bg-black-200 px-1 py-1 rounded-md font-mono text-sm">
            Ctrl + ,
        </div>
    </div>
    <div class="flex justify-between">
        <p>{$t("settings.shortcuts.shortcut.commandPalette")}</p>
        <div class="bg-black-200 px-1 py-1 rounded-md font-mono text-sm">
            Ctrl + P
        </div>
    </div>
</div>