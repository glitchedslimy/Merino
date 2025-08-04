<script lang="ts">
    import { fly } from "svelte/transition";
    import {Button, Icon} from "@atoms";

    let isDropdownOpen = $state(false);
    let { currentSpaceName, availableSpaces, onSpaceSelected } = $props();

    const handleDropdownClick = () => {
        isDropdownOpen = !isDropdownOpen;
    }
    

    function selectSpace(space) {
        onSpaceSelected(space);
        isDropdownOpen = false;
    }

    
    const handleDropdownFocusLoss = (event: FocusEvent) => {
        if (event.relatedTarget instanceof HTMLElement && event.currentTarget instanceof HTMLElement) {
            if (event.currentTarget.contains(event.relatedTarget)) {
                return; 
            }
        }
        isDropdownOpen = false;
    };
</script>

<div class="flex justify-between items-center">
    <div
        class="dropdown relative"
        onfocusout={handleDropdownFocusLoss}
        tabindex="-1"
    >
        <Button type="button" dropdown handleClick={handleDropdownClick}>
            {currentSpaceName}
            <Icon iconName="chevron-down" className={`${isDropdownOpen ? 'rotate-180' : 'rotate-0'} duration-400 transition-all`}/>
        </Button>

        {#if isDropdownOpen}
            <ul
                class="bg-black-100 shadow rounded-box absolute px-5 py-2 rounded-md z-10"
                transition:fly={{duration:60}}
                role="menu"
                aria-orientation="vertical"
            >
            {#each availableSpaces as space (space.id)}
               <li class="dropdown-item" class:selected={space.name === currentSpaceName} onclick={() => selectSpace(space)}>
                    {space.name}
               </li>
            {/each}
            </ul>
        {/if}
    </div>
</div>