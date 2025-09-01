<script lang="ts">
    import { fly } from "svelte/transition";
    import { onMount, tick, onDestroy } from "svelte";
    import { showCommandPalette } from "../../lib/stores/commandpalette/commandpalette";

    let searchTerm = "";
    let inputElement: HTMLInputElement;
    let selectedIndex = -1;

    const commands = [
        { name: "New Note", action: () => console.log('New Note') },
        { name: "Export Notes", action: () => console.log('Export Notes') },
        { name: "Settings", action: () => console.log('Open Settings') },
        { name: "Change Theme", action: () => console.log('Change Theme') },
        { name: "Share Note", action: () => console.log('Share Note') },
        { name: "Delete Note", action: () => console.log('Delete Note') },
        { name: "About", action: () => console.log('About App') },
    ];

    $: filteredCommands = commands.filter((cmd) =>
        cmd.name.toLowerCase().includes(searchTerm.toLowerCase())
    );

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") {
            closePalette();
        } else if (e.key === "ArrowUp") {
            e.preventDefault();
            selectedIndex = (selectedIndex - 1 + filteredCommands.length) % filteredCommands.length;
        } else if (e.key === "ArrowDown") {
            e.preventDefault();
            selectedIndex = (selectedIndex + 1) % filteredCommands.length;
        } else if (e.key === "Enter") {
            e.preventDefault();
            if (selectedIndex !== -1) {
                filteredCommands[selectedIndex].action();
                closePalette();
            }
        }
    }

    function closePalette() {
        $showCommandPalette = false;
    }

    onMount(async () => {
        await tick();
        if (inputElement) {
            inputElement.focus();
        }
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });
</script>

<div
    transition:fly={{ duration: 200 }}
    class="bg-black-100 h-fit max-h-[500px] w-[500px] rounded-md shadow-2xl px-xs py-xs overflow-hidden"
    role="dialog"
    aria-label="Command Palette"
>
    <input
        type="text"
        class="w-full px-2 py-1 bg-black-100 border border-black-200 rounded-md mb-xs"
        placeholder=">"
        bind:value={searchTerm}
        bind:this={inputElement}
    />
    <ul class="overflow-y-auto max-h-[450px]">
        {#if filteredCommands.length > 0}
            {#each filteredCommands as command, i}
                <li
                    class="px-2 py-1 cursor-pointer text-sm"
                    class:bg-black={i === selectedIndex}
                    class:text-white={i === selectedIndex}
                    class:text-black-300={i !== selectedIndex}
                    onclick={() => {
                        command.action();
                        closePalette();
                    }}
                    onmouseenter={() => (selectedIndex = i)}
                >
                    {command.name}
                </li>
            {/each}
        {:else}
            <li class="px-2 py-1 text-black-300 text-sm italic">No commands found.</li>
        {/if}
    </ul>
</div>