<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import {
        createSpace,
        listSpaces,
        deleteSpace,
    } from "@services/internal/api/tauri-commands";
    import { addToast } from "@stores/toast-store";
    import { openAdminSpaces, spacesStore } from "@stores/workspace-store";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";

    let openCreateSpace = $state(false);
    let spaceName = $state("My space name");
    let activeDropdown = $state<string | null>(null);

    async function loadSpaces() {
        try {
            const loadedSpaces = await listSpaces();
            spacesStore.update((s) => ({ ...s, spaces: loadedSpaces })); // Automatically set the first space as active
            if (loadedSpaces.length > 0) {
                spacesStore.update((s) => ({
                    ...s,
                    activeSpaceName: loadedSpaces[0].name,
                }));
            }
        } catch (e) {
            console.error("Failed to list spaces:", e);
        }
    }

    function closeAdminSpace() {
        openAdminSpaces.set(false);
    }

    function openCreateSpaces() {
        openCreateSpace = !openCreateSpace;
    }

    function toggleSpaceOptions(spaceName: string) {
        if (activeDropdown === spaceName) {
            activeDropdown = null;
        } else {
            activeDropdown = spaceName;
        }
    }

    async function handleSpaceCreation() {
        if (spaceName.trim() !== "") {
            try {
                await createSpace({
                    name: spaceName.trim(),
                });
                addToast("Successfully created space", "success");
                // Reload the spaces list after successful creation
                await loadSpaces();
                // Optionally, reset the input and close the creation view
                spaceName = ""; // Clear the input field
                openCreateSpace = false; // Close the creation section
            } catch (error) {
                console.error("Failed to create space:", error);
                addToast("Failed to create space", "error");
            }
        }
    }

    async function handleDeleteSpace(spaceToDelete: string) {
        try {
            await deleteSpace(spaceToDelete);
            addToast(`Successfully deleted space: ${spaceToDelete}`, "success");
            await loadSpaces();
            activeDropdown = null; // Close the dropdown after deletion
        } catch (error) {
            console.error("Failed to delete space:", error);
            addToast(`Failed to delete space: ${spaceToDelete}`, "error");
        }
    }

    onMount(() => {
        loadSpaces();
    });
</script>

<div
    class="absolute inset-0 flex items-center justify-center bg-black-200/40 backdrop-blur-xs z-20 overflow-hidden"
>
    <div class="bg-black-100 px-sm py-sm rounded-md w-full mx-xl">
        <div class="flex justify-between items-center mb-sm px-md py-md">
            <h1 class="text-lg font-medium">Space admin</h1>
            <Button intent="icon" class="h-fit" handleClick={closeAdminSpace}>
                <Icon iconName="close" />
            </Button>
        </div>
        <div class="flex space-x-xs">
            <div class="bg-black px-lg py-lg rounded-md flex-1">
                <h1 class="text-xl font-bold mb-md">Spaces</h1>
                <section class="h-96 overflow-y-auto">
                    {#each $spacesStore.spaces as space}
                        <div
                            class="flex justify-between items-center hover:bg-brand-primary px-sm py-sm rounded-md text-white-400 hover:text-white duration-200 transition my-sm"
                        >
                            <div>
                                <h2>{space.name}</h2>
                                <h3>
                                    {space.route?.includes("com.merino")
                                        ? "Default"
                                        : space.route}
                                </h3>
                            </div>
                            <div class="relative">
                                <Button
                                    intent="icon"
                                    handleClick={() =>
                                        toggleSpaceOptions(space.name)}
                                >
                                    <Icon iconName="dotsvertical" />
                                </Button>

                                {#if activeDropdown === space.name}
                                    <div
                                        class="absolute right-0 w-48 rounded-md shadow-lg bg-black-100 ring-1 ring-black ring-opacity-5"
                                    >
                                        <ul class="py-xs px-xs">
                                            <li>
                                                <Button
                                                    intent="notes"
                                                    class="hover:text-white hover:bg-utils-red transition duration-200 cursor-pointer"
                                                    handleClick={() =>
                                                        handleDeleteSpace(
                                                            space.name,
                                                        )}
                                                >
                                                    Delete space
                                                </Button>
                                            </li>
                                        </ul>
                                    </div>
                                {/if}
                            </div>
                        </div>
                    {/each}
                </section>
            </div>
            <div class="bg-black px-lg py-lg rounded-md flex-1">
                <section class="flex justify-center items-center flex-col">
                    <img
                        src="./src/assets/Wooly.svg"
                        alt="wooly logo"
                        width="100"
                    />
                    <h1 class="text-2xl font-bold">Merino</h1>
                    <p class="text-sm font-medium text-white-400">
                        Version 0.1.0
                    </p>
                    {#if !openCreateSpace}
                        <div class="w-full">
                            <div
                                class="flex justify-between w-full mt-lg px-xl"
                            >
                                <div>
                                    <p class="font-medium">
                                        Create a new space
                                    </p>
                                    <p class="text-sm text-white-400">
                                        Create a new space of Merino in a
                                        folder.
                                    </p>
                                </div>
                                <Button
                                    intent="primary"
                                    handleClick={openCreateSpaces}
                                >
                                    Create
                                </Button>
                            </div>
                            <div
                                class="flex justify-between w-full mt-lg px-xl"
                            >
                                <div>
                                    <p class="font-medium">Open a space</p>
                                    <p class="text-sm text-white-400">
                                        Open an already existing space of
                                        Merino. (Disabled)
                                    </p>
                                </div>
                                <Button intent="gray">Open</Button>
                            </div>
                        </div>
                    {/if}
                    {#if openCreateSpace}
                        <div class="w-full justify-between">
                            <Button
                                intent="ghost"
                                handleClick={openCreateSpaces}
                            >
                                Back
                            </Button>
                            <div
                                class="flex justify-between w-full mt-lg px-xl"
                            >
                                <div>
                                    <p class="font-medium">Name of the space</p>
                                    <p class="text-sm text-white-400">
                                        Pick a name for your new awesome space.
                                    </p>
                                </div>
                                <input
                                    type="text"
                                    placeholder="Space name"
                                    bind:value={spaceName}
                                    class="bg-black-100 px-sm py-sm rounded-md text-black-200 focus:text-white"
                                />
                            </div>
                        </div>
                        <div class="w-full justify-between text-black-200">
                            <div
                                class="flex justify-between w-full mt-lg px-xl"
                            >
                                <div>
                                    <p class="font-medium">
                                        Ubication of the space
                                    </p>
                                    <p class="text-sm">
                                        For now this option is disabled, does
                                        not work, but is included for the
                                        future.
                                    </p>
                                </div>
                                <input
                                    type="text"
                                    placeholder="Space name"
                                    bind:value={spaceName}
                                    class="bg-black-100 px-sm py-sm rounded-md text-black-200 focus:text-white"
                                    disabled
                                />
                            </div>
                        </div>
                        <div class="mt-lg">
                            <Button
                                intent="primary"
                                class="w-full"
                                handleClick={handleSpaceCreation}
                            >
                                Create Space
                            </Button>
                        </div>
                    {/if}
                </section>
            </div>
        </div>
    </div>
</div>
