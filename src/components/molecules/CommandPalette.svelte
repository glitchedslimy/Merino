<script lang="ts">
    import { fly } from "svelte/transition";
    import { onMount, onDestroy, tick } from "svelte";
    import { invoke } from "@tauri-apps/api/core"; // Corrected import path
    import { showCommandPalette } from "../../lib/stores/commandpalette/commandpalette";
    import { openNote } from "../../lib/actions/editor/notes-buffer";

    // Utility function to debounce API calls
    function debounce<T extends (...args: any[]) => void>(func: T, delay: number): (...args: Parameters<T>) => void {
        let timeout: ReturnType<typeof setTimeout>;
        return (...args: Parameters<T>) => {
            clearTimeout(timeout);
            timeout = setTimeout(() => func(...args), delay);
        };
    }

    // New state variables using Svelte 5 runes
    let searchTerm = $state("");
    let inputElement: HTMLElement | null = $state(null);
    let selectedIndex = $state(0);
    let searchResults = $state<{ name: string; action: () => void }[]>([]);
    let isSearching = $state(false);

    // Hardcoded commands as before
    const commands = [
        { name: "New Note", action: () => console.log('New Note') },
        { name: "Export Notes", action: () => console.log('Export Notes') },
        { name: "Settings", action: () => console.log('Open Settings') },
        { name: "Change Theme", action: () => console.log('Change Theme') },
        { name: "Share Note", action: () => console.log('Share Note') },
        { name: "Delete Note", action: () => console.log('Delete Note') },
        { name: "About", action: () => console.log('About App') },
    ];

    // Derived reactive states using $derived
    let filteredCommands = $derived(
        commands.filter((cmd) => cmd.name.toLowerCase().includes(searchTerm.toLowerCase()))
    );

    let displayItems = $derived([...filteredCommands, ...searchResults]);

    // Debounced search function
    const debouncedSearch = debounce(async (query: string) => {
        if (!query.trim()) {
            searchResults = [];
            isSearching = false;
            return;
        }

        isSearching = true;
        try {
            const results = await invoke("search_notes_cmd", { query });
            console.log(results)
            searchResults = results.map((note) => ({
                name: note.name,
                action: () => {
                    openNote(note);
                }
            }));
        } catch (error) {
            console.error("Search failed:", error);
            searchResults = [];
        } finally {
            isSearching = false;
        }
    }, 300);

    // Run a side effect whenever searchTerm changes
    $effect(() => {
        debouncedSearch(searchTerm);
    });

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") {
            closePalette();
        } else if (e.key === "ArrowUp") {
            e.preventDefault();
            selectedIndex = (selectedIndex - 1 + displayItems.length) % displayItems.length;
        } else if (e.key === "ArrowDown") {
            e.preventDefault();
            selectedIndex = (selectedIndex + 1) % displayItems.length;
        } else if (e.key === "Enter") {
            e.preventDefault();
            if (selectedIndex !== -1) {
                displayItems[selectedIndex].action();
                closePalette();
            }
        }
    }

    function closePalette() {
        showCommandPalette.set(false);
        searchTerm = "";
        searchResults = [];
        selectedIndex = 0;
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
        placeholder="Search a note or use a command."
        bind:value={searchTerm}
        bind:this={inputElement}
    />
    <ul class="overflow-y-auto max-h-[450px]">
        {#if displayItems.length > 0}
            {#each displayItems as item, i}
                <li
                    class="px-2 py-1 cursor-pointer text-sm"
                    class:bg-black={i === selectedIndex}
                    class:text-white={i === selectedIndex}
                    class:text-black-300={i !== selectedIndex}
                    onclick={() => {
                        item.action();
                        closePalette();
                    }}
                    onmouseenter={() => (selectedIndex = i)}
                >
                    {item.name}
                </li>
            {/each}
        {:else if isSearching}
            <li class="px-2 py-1 text-black-300 text-sm italic">Searching...</li>
        {:else}
            <li class="px-2 py-1 text-black-300 text-sm italic">No commands or notes found.</li>
        {/if}
    </ul>
</div>