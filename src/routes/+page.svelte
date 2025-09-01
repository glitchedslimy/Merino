<script lang="ts">
    import "../assets/styles/tw.css";
    import Appbar from "@components/organisms/Appbar.svelte";
    import Editor from "@components/templates/Editor.svelte";
    import Sidebar from "@components/organisms/Sidebar.svelte";
    import Editorbar from "@components/organisms/Editorbar.svelte";
    import Workspace from "@components/templates/Workspace.svelte";
    import Ai from "@components/templates/AI.svelte";
    import CommandPaletteSpace from "@components/organisms/CommandPaletteSpace.svelte";
    import { onDestroy, onMount } from "svelte";
    import { showCommandPalette } from "../lib/stores/commandpalette/commandpalette";

    function handleGlobalKeydown(event: KeyboardEvent) {
        // Check for Ctrl/Cmd + K
        const isCtrlK = event.key === "k" && event.ctrlKey;
        const isCmdK = event.key === "k" && event.metaKey;

        if (isCtrlK || isCmdK) {
            // Prevent the default browser action for Ctrl/Cmd + K
            event.preventDefault();

            // Toggle the showCommandPalette store
            $showCommandPalette = !$showCommandPalette;
        }
    }

    onMount(() => {
        // Add the global event listener when the component is mounted
        window.addEventListener("keydown", handleGlobalKeydown);
    });

    onDestroy(() => {
        // Remove the listener when the component is destroyed
        window.removeEventListener("keydown", handleGlobalKeydown);
    });
</script>

<main class="bg-black-100 text-white h-screen flex flex-col">
    <Appbar />
    <CommandPaletteSpace />
    <section class="flex flex-1 min-h-0">
        <Sidebar />
        <div class="flex-1 flex flex-col min-w-0">
            <div class="flex flex-1 min-h-0">
                <Workspace />
                <!-- Resizer here -->
                <div
                    class="flex-1 flex flex-col overflow-hidden editor-wrapper"
                >
                    <Editor />
                </div>
            </div>
            <Editorbar />
        </div>
        <!-- Resizer here -->
        <Ai />
    </section>
</main>
