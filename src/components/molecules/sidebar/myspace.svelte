<script>
    import Button from "../../atoms/button/button.svelte";
    import Icon from "../../atoms/icon/icon.svelte";
    let { handleMySpaceClose, closeMySpace = $bindable() } = $props()
</script>

<div class="flex flex-col space-y-5">
    <div class="flex items-center justify-between w-50">
        <div class="flex space-x-2 items-center">
            <span><Icon iconName="home" /></span>
            <p class="font-bold">Mi Espacio</p>
            <span>
                <Icon iconName="chevron-down" />
            </span>
        </div>
        <span>
            <Button type="button" icon handleClick={handleMySpaceClose}>
                <Icon iconName="chevron-pipe-left" />
            </Button>
        </span>
    </div>
    <div>
        <ul>
            <li class="text-[#949494] flex space-x-2 items-center">
                <Icon iconName="ai" />
                <p>Notalia AI</p>
            </li>
            <li class="text-[#949494] flex space-x-2 items-center">
                <Icon iconName="templates" />
                <p>Plantillas</p>
            </li>
        </ul>
    </div>
</div>
