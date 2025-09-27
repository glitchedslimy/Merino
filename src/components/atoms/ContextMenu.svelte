<script lang="ts">
    import { t } from "$lib/i18n";
    import Icon from "@components/atoms/Icon.svelte";
    import { slide } from "svelte/transition";

    let {
        x,
        y,
        menuItems,
        contextId = "",
        onclose = () => {},
    } = $props();

    let menuElement: HTMLDivElement | undefined = undefined;

    $effect(() => {
        if (menuElement) {
            const menuRect = menuElement.getBoundingClientRect();
            const viewportWidth = window.innerWidth;
            const viewportHeight = window.innerHeight;

            let newX = x;
            let newY = y;

            if (menuRect.right > viewportWidth) {
                newX = viewportWidth - menuRect.width - 10;
            }
            if (menuRect.bottom > viewportHeight) {
                newY = viewportHeight - menuRect.height - 10;
            }

            if (newX < 0) newX = 10;
            if (newY < 0) newY = 10;

            menuElement.style.left = `${newX}px`;
            menuElement.style.top = `${newY}px`;
        }
    });

    $effect.root(() => {
        function handleClickOutside(event: MouseEvent) {
            if (menuElement && !menuElement.contains(event.target as Node)) {
                onclose();
            }
        }

        document.addEventListener("click", handleClickOutside);

        return () => {
            document.removeEventListener("click", handleClickOutside);
        };
    });

    function handleMenuItemClick(action: (id: string) => void) {
        action(contextId);
        onclose();
    }
</script>

<div
    transition:slide={{ duration: 100, axis: "y"}}
    bind:this={menuElement}
    class="absolute bg-black-100 flex flex-col px-xs py-xs rounded-md z-20"
    style="left: {x}px; top: {y}px;"
>
    {#each menuItems as item}
        <button
            onclick={() => handleMenuItemClick(item.action)}
            class="px-2 py-2 text-left rounded-sm hover:bg-black-200 flex items-center space-x-xs"
        >
            <Icon iconName={item.icon} width="20" />
            <span>{$t(item.label)}</span>
        </button>
    {/each}
</div>