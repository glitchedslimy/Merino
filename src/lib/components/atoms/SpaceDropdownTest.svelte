<script lang="ts">
    import { listSpaces } from '@services/internal/api/tauri-commands';
    import { loadPersistentState, spacesStore, activeSpaceName, saveActiveSpaceName } from '@stores/workspace-store';
    import { onMount } from 'svelte';
    import { fade, slide } from 'svelte/transition';
    import Button from './Button.svelte';

    let isOpen = $state(false);
    
    // We can use a reactive derived value for the selected option
    let selectedOption = $derived($activeSpaceName || 'Mi Espacio');
    
    // We'll use a derived state to get the list of spaces reactively
    let spaces = $derived($spacesStore.spaces);

    async function loadSpaces() {
        try {
            const loadedSpaces = await listSpaces();
            spacesStore.set({ spaces: loadedSpaces }); // Correctly set the entire store object
        } catch (e) {
            console.error('Failed to list spaces:', e);
        }
    }

    function toggleDropdown() {
      isOpen = !isOpen;
      console.log(isOpen)
    }

    function selectOption(spaceName: string) {
        activeSpaceName.set(spaceName);
        saveActiveSpaceName(spaceName);
        isOpen = false;
    }

    function openAdminSpace() {
        // Your logic here
    }

    // Use an effect to load the spaces once the component is created
    $effect(() => {
        loadSpaces();
    });
</script>

<div class="relative w-full mx-auto">
    <button
        onclick={toggleDropdown}
        class="w-full flex items-center justify-between space-x-md px-sm py-sm text-left bg-transparent rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-brand-primary focus:border-brand-primary transition-all duration-300 ease-in-out hover:bg-black-200 group"
    >
        <span class="text-white">{selectedOption}</span>
        <svg
            class="h-5 w-5 text-gray-400 transform transition-transform duration-300"
            class:rotate-180={isOpen}
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
        >
            <path
                fill-rule="evenodd"
                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                clip-rule="evenodd"
            />
        </svg>
    </button>

    {#if isOpen}
        <div
            transition:slide={{ duration: 200 }}
            class="absolute z-10 mt-1 bg-black-100 rounded-md shadow-lg transition-all duration-300 ease-in-out"
        >
            <ul
                class="py-1"
                transition:fade={{ duration: 150, delay: 50 }}
            >
                {#each spaces as option}
                    <li>
                        <Button intent="ghost" handleClick={() => selectOption(option.name)} 
                       class={option.name === $activeSpaceName ? 'bg-brand-primary text-white' : ''}>
                          {option.name}
                        </Button>
                    </li>
                {/each}
                <hr class="border border-black-200"/>
                <li>
                  <Button intent="ghost" handleClick={openAdminSpace}>
                    Administrar espacios
                  </Button>
                </li>
            </ul>
        </div>
    {/if}
</div>