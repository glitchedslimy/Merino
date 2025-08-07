<script lang="ts">
  import EditorJS, { type OutputData } from '@editorjs/editorjs';
  import { initEditor } from '@services/internal/editor/initEditor';
  import { activeSpaceName } from '@stores/workspace-store';
  import { onDestroy, onMount } from 'svelte';
  import { get } from 'svelte/store';
  
  let { noteName, initialContent } = $props();
  let editorInstance: EditorJS | null = null;
  const editorHolderId = 'editorjs';

  $effect(() => {
    if (editorInstance && typeof editorInstance.destroy === 'function') {
      editorInstance.destroy();
      editorInstance = null;
    }

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

<div id="{editorHolderId}" class="bg-black overflow-y-auto mt-xs rounded-md flex-1">
</div>