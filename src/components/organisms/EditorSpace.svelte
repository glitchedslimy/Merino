<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import EditorJS, { type OutputData } from "@editorjs/editorjs";
    import { get } from "svelte/store";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import { initializeEditor } from "../../lib/actions/editor/initializeEditor";
    import { activeNoteName } from "../../lib/stores/workspace/notes-store";
    import { getNoteContent } from "../../lib/api/tauri/get/notes-api-get";
    import { marked } from 'marked'; // <-- Add this import
    import { renderer } from "../../lib/actions/editor/renderer/marked";
    
    let { noteName, initialContent, editorHolderId = 'editorjs-container'} = $props();

    let reloadDebounceTimeout: any = null;
    let editorInstance: EditorJS | null = null;
    let isLoadingContent = $state(false);
    let htmlString = $state("")
    async function reloadEditor() {
        let contentToLoad: OutputData | null = null;
        
        // 1. Get the markdown content from your API
        if (noteName && get(activeSpace)) {
            isLoadingContent = true;
            try {
                const markdownString = await getNoteContent(get(activeSpace) ?? '', noteName);
                console.log(markdownString)
                marked.use({ renderer })
                htmlString = await marked.parse(markdownString.content);
                const cleanedHtmlString = htmlString.replace(/\n/g, '');
                htmlString = cleanedHtmlString;
                contentToLoad = null;
            } catch(e) {
                console.error("Failed to load content:", e);
            } finally {
                isLoadingContent = false;
            }
        }
        
        // 4. Destroy the existing instance
        if (editorInstance && typeof editorInstance.destroy === 'function') {
            editorInstance.destroy();
            editorInstance = null;
        }
        console.log(htmlString)
        // 5. Re-initialize the editor with a placeholder and render the content
        if (noteName && get(activeSpace)) {
            editorInstance = initializeEditor(
                editorHolderId,
                get(activeSpace) ?? '',
                noteName,
                get(activeNoteName) ?? '',
                htmlString
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
        window.addEventListener('resize', debounceReload);
    });

    onDestroy(() => {
        if(editorInstance && typeof editorInstance.destroy === 'function') {
            editorInstance.destroy();
        }
        window.removeEventListener('resize', reloadEditor);
    });
</script>

<div id="{editorHolderId}">
    {#if isLoadingContent}
    <p>Loading...</p>
    {/if}
</div>