<script>
    import Icon from "../../atoms/icon/icon.svelte";
    import Button from "../../atoms/button/button.svelte";
    let { editor, changeToReadOnlymode, readOnly = $bindable(), closeMySpace, handleOpenMySpace } = $props();

</script>

<div class="flex justify-between flex-row bg-black py-2 px-3">
    <div class="flex space-x-1 items-center">
        {#if closeMySpace}
            <Button type="button" icon handleClick={handleOpenMySpace}>
                <Icon iconName="chevron-pipe-right" />
            </Button>
        {/if}
        <Icon iconName="back" />
        <Icon iconName="forward" />
    </div>
    <div>
        <p class="text-sm text-gray-400">Mi Espacio/Nueva Nota</p>
    </div>
    <div class="flex space-x-2 items-center">
        <Button type="button" icon handleClick={changeToReadOnlymode}>
            {#if readOnly}
                <Icon iconName="edit" />
            {:else}
                <Icon iconName="read" />
            {/if}
        </Button>
        <Icon iconName="more" />
    </div>
</div>