<script lang="ts">
    import EditorJS, { type OutputData } from '@editorjs/editorjs';
    import { initEditor } from '@services/internal/editor/initEditor';
    import { activeSpaceName } from '@stores/workspace-store';
    import { onDestroy, onMount } from 'svelte';
    import { get } from 'svelte/store';
    
    let { noteName, initialContent } = $props();
    let editorInstance: EditorJS | null = null;
    const editorHolderId = 'editorjs';

    // This effect now ONLY depends on 'noteName' and '$activeSpaceName'.
    // 'initialContent' is not in the dependency list.
    // This breaks the reactive loop.
    $effect(() => {
        // Destroy the old instance if a new note is selected
        if (editorInstance && typeof editorInstance.destroy === 'function') {
            editorInstance.destroy();
            editorInstance = null;
        }

        // Initialize a new editor instance with the correct content
        if (noteName && $activeSpaceName) {
            editorInstance = initEditor(
                editorHolderId,
                $activeSpaceName ?? '',
                noteName,
                initialContent
            );
        }
    });

    onDestroy(() => {
        if (editorInstance && typeof editorInstance.destroy === 'function') {
            editorInstance.destroy();
        }
    });
</script>

<div id="{editorHolderId}" class="bg-black h-screen overflow-y-auto px-4 md:px-4 xl:px-0">
</div>