<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import EditorJS from "@editorjs/editorjs";
    import { get } from "svelte/store";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import { initializeEditor } from "../../lib/actions/editor/initializeEditor";
    import { getNoteContent } from "../../lib/api/tauri/get/notes-api-get";

    let {
        noteName,
        editorHolderId = "editorjs-container",
        noteFolder,
    } = $props();

    let reloadDebounceTimeout: any = null;
    let editorInstance: EditorJS | null = null;
    let isLoadingContent = $state(false);
    let htmlString = $state("");

    async function reloadEditor() {

        if (noteName && get(activeSpace)) {
            isLoadingContent = true;
            try {

                const noteData = await getNoteContent(
                    get(activeSpace) ?? "",
                    noteName,
                    noteFolder,
                );
                htmlString = noteData.content;
            } catch (e) {
                console.error("Failed to load content:", e);
            } finally {
                isLoadingContent = false;
            }
        }

        if (editorInstance && typeof editorInstance.destroy === "function") {
            editorInstance.destroy();
            editorInstance = null;
        }
        if (noteName && get(activeSpace)) {
            editorInstance = initializeEditor(
                editorHolderId,
                get(activeSpace) ?? "",
                noteName,
                htmlString,
                noteFolder,
            );
        }
    }

    $effect(() => {
        reloadEditor();
    });

    onMount(() => {
        const debounceReload = () => {
            clearTimeout(reloadDebounceTimeout);
            reloadDebounceTimeout = setTimeout(reloadEditor, 200);
        };
        window.addEventListener("resize", debounceReload);
    });

    onDestroy(() => {
        if (editorInstance && typeof editorInstance.destroy === "function") {
            editorInstance.destroy();
        }
        window.removeEventListener("resize", reloadEditor);
    });
</script>

<div id={editorHolderId}>
    {#if isLoadingContent}
        <p>Loading...</p>
    {/if}
</div>
