<script lang="ts">
    import EditorJS, { type OutputData } from '@editorjs/editorjs';
    import { initEditor } from '@services/internal/editor/initEditor';
    import { activeNoteName, activeSpaceName } from '@stores/workspace-store';
    import { onDestroy, onMount } from 'svelte';
    import { get } from 'svelte/store';
    
    let { noteName, initialContent, editorHolderId = 'editorjs-container' } = $props();
    let editorInstance: EditorJS | null = null;
    let reloadDebounceTimeout: any = null;

    async function reloadEditor() {
        let currentContent: OutputData | null = null;
        
        // 1. Save the current content before destroying the instance
        if (editorInstance && typeof editorInstance.save === 'function') {
            try {
                currentContent = await editorInstance.save();
            } catch (error) {
                console.error("Failed to save Editor.js content:", error);
                // Fallback to initial content if save fails
                currentContent = initialContent;
            }
        }
        
        // 2. Destroy the existing instance
        if (editorInstance && typeof editorInstance.destroy === 'function') {
            editorInstance.destroy();
            editorInstance = null;
        }

        // 3. Re-initialize the editor with the saved content
        if (noteName && get(activeSpaceName)) {
            editorInstance = initEditor(
                editorHolderId,
                get(activeSpaceName) ?? '',
                noteName,
                get(activeNoteName) ?? '', // Correctly passing the note name
                currentContent || initialContent // Use saved content or initial content as fallback
            );
        }
    }

    $effect(() => {
        reloadEditor();
    });

    onMount(() => {
        const debouncedReload = () => {
            clearTimeout(reloadDebounceTimeout);
            reloadDebounceTimeout = setTimeout(reloadEditor, 200); // 200ms debounce
        };
        window.addEventListener('resize', debouncedReload);
    });

    onDestroy(() => {
        if (editorInstance && typeof editorInstance.destroy === 'function') {
            editorInstance.destroy();
        }
        window.removeEventListener('resize', reloadEditor);
    });
</script>

<div id="{editorHolderId}" class="bg-black overflow-y-auto overflow-x-hidden mt-lg rounded-md flex-1">
</div>