<script>
    import { Button, Icon } from "@atoms";
    import { showWorkspace, activeSpaceName, activeNote } from '@stores/workspace-store'
    import { handleOpenMySpace } from '@services/internal/workspace/control-hide-workspace'
</script>

<div class="flex justify-between flex-row bg-black py-2 px-3 rounded-md">
    <div class="flex space-x-xs items-center">
        {#if !$showWorkspace}
            <Button intent="icon" handleClick={handleOpenMySpace}>
                <Icon iconName="chevron-pipe-right" />
            </Button>
        {/if}
        <Icon iconName="back" width="20"/>
        <Icon iconName="forward" width="20"/>
    </div>
    <div>
        <p class="text-sm text-gray-400">{$activeSpaceName}/{$activeNote?.name}</p>
    </div>
    <div class="flex space-x-sm items-center">
        <Button intent="icon">
                <Icon iconName="edit" width="20"/>
        </Button>
        <Icon iconName="more" width="20"/>
    </div>
</div>