<script lang="ts">
    import Button from '@components/atoms/Button.svelte';
    import Icon from '@components/atoms/Icon.svelte';
    import { listSpaces } from '@services/internal/api/tauri-commands'
    import { openAdminSpaces, spacesStore } from '@stores/workspace-store';
    import { onMount } from 'svelte';
    import { get } from 'svelte/store';
    async function loadSpaces() {
    try {
        const loadedSpaces = await listSpaces();
        spacesStore.update(s => ({ ...s, spaces: loadedSpaces }));
        // Automatically set the first space as active
        if (loadedSpaces.length > 0) {
            spacesStore.update(s => ({ ...s, activeSpaceName: loadedSpaces[0].name }));
        }
    } catch (e) {
        console.error('Failed to list spaces:', e);
    }
}

function closeAdminSpace() {
    openAdminSpaces.set(false)
}

onMount(() => {
    loadSpaces()
})
</script>
<div class="absolute w-full h-screen flex items-center justify-center bg-black/90 z-20">
    <div class="bg-black-100 mx-2xl rounded-md flex justify-end">
        <div class="flex">
            <div class="bg-black-200 px-lg py-lg rounded-tl-md rounded-bl-md">
                <h1 class="text-2xl font-black">Espacios</h1>
                <ul class="flex flex-col space-y-sm mt-md">   
                    {#each get(spacesStore).spaces as space, index}
                    <li>
                        <p class="text-lg font-bold">{space.name}</p>
                        <p class="text-sm text-white-400">{space.route}</p>
                    </li>
                    {/each}
                </ul>
            </div>
            <div class="px-lg py-lg flex flex-col justify-end">
                <Button intent="icon" class="absolute right-64 top-84" handleClick={closeAdminSpace}>
                    <Icon iconName="close" />
                </Button>
                <div>
                    <h1 class="text-2xl text-center font-bold">Notalia</h1>
                    <h2 class="text-sm text-center text-white-400">Versión 0.1.0</h2>
                    <div class="flex mt-lg">
                        <div class="w-3xl">
                            <p>Crear espacio nuevo</p>
                            <p class="text-sm text-white-400">Crea un nuevo espacio de Notalia en un directorio.</p>
                        </div>
                        <Button intent="primary">Crear</Button>
                    </div>
                    <div class="flex mt-lg">
                        <div class="w-3xl">
                            <p>Abrir directorio como espacio</p>
                            <p class="text-sm text-white-300">Abre un directorio ya existente como un espacio de Notalia.</p>
                        </div>
                        <Button intent="gray">Abrir</Button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>