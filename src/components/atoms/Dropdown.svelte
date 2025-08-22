<script lang="ts" generics="T">
    import { slide } from "svelte/transition";
    import Button from "./Button.svelte";
    import Icon from "./Icon.svelte";
    import type { IconName } from "../../lib/types/IconName";
    import { ellipsisTooltip } from "../../lib/useHooks/ellipsisTooltip";
    import {
        handleHideTooltip,
        handleShowTooltip,
    } from "../../lib/actions/workspace/notes";
    import { getModelLogo } from "../../lib/actions/ai/getModelLogo";

    let {
        options,
        onSelect,
        selectedOption,
        icon,
        svgData
    }: {
        options: any;
        onSelect?: (option: T) => void;
        selectedOption: string;
        icon?: IconName;
        svgData?: string;
    } = $props();

    let isOpen = $state(false);
</script>

<div class="relative w-fit max-auto">
    <Button intent="dropdown" onClick={() => (isOpen = !isOpen)}>
        {#if icon}
            <Icon iconName={icon ? icon : undefined} width="20" class="text-brand-primary-light" />
        {/if}
        {#if svgData}
            <img src={svgData} alt="model logo" />
        {/if}
        <span
            use:ellipsisTooltip={{
                onHide: handleHideTooltip,
                onShow: handleShowTooltip,
            }}
            class="overflow-hidden text-ellipsis whitespace-nowrap w-[6rem]"
            >{selectedOption}</span
        >
        <Icon iconName="chevron-down" width="20" />
    </Button>
    {#if isOpen}
        <div
            class="absolute w-full z-10 mt-1 bg-black-100 shadow-lg transition-all duration-300 ease-in-out rounded-bl-md rounded-br-md"
            transition:slide={{ duration: 200, axis: "y" }}
        >
            <ul>
                {#each options as option}
                    <li>
                        <Button
                            intent="dropdownText"
                            onClick={() => {
                                if (onSelect) {
                                    onSelect(option.name);
                                    isOpen = false;
                                }
                            }}
                            class="w-full flex gap-x-sm overflow-hidden text-ellipsis whitespace-nowrap"
                        >
                        {#if svgData}
                            <img src={getModelLogo(option.name)} alt="model logo">
                        {/if}
                            <span class="overflow-hidden text-ellipsis whitespace-nowrap">{option.name}</span>
                        </Button>
                    </li>
                {/each}
            </ul>
        </div>
    {/if}
</div>
