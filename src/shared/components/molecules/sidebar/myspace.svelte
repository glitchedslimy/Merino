<script>
    import {Button, Icon} from '@atoms'
    import { Dropdown } from "@molecules";
    let { closeMySpace = $bindable(), currentSpaceName, availableSpaces, onSpaceSelected } = $props()
     import { handleCloseMySpace } from '@scripts/mySpace/mySpace'
</script>

<div class="flex flex-col space-y-5">
    <div class="flex items-center justify-between w-50">
        <div class="flex space-x-2 items-center">
            <span><Icon iconName="home" /></span>
            <Dropdown {currentSpaceName} {availableSpaces} {onSpaceSelected}/>
        </div>
        <span>
            <Button type="button" icon handleClick={handleCloseMySpace}>
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
