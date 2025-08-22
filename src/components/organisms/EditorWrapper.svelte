<script lang="ts">
    import type { OutputData } from "@editorjs/editorjs";
    import { activeNoteName, opennedNotes } from "../../lib/stores/workspace/notes-store";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import EditorSpace from "./EditorSpace.svelte";
    import { updateNoteContent } from "../../lib/api/tauri/update/notes-api-update";
    import { getNoteContent } from "../../lib/api/tauri/get/notes-api-get";
    import NoNotesInSpace from "./NoNotesInSpace.svelte";

  // The active note is now derived from the activeNoteName
  let activeNote = $derived($opennedNotes.find(note => note.name === $activeNoteName));
  let displayedContent = $state<OutputData | null>(null);
  let isLoadingContent = $state(false);

  // Function to handle content changes and save the note
  async function handleContentChange(event: CustomEvent<OutputData>) {
    const newContent = event.detail;
    if (!activeNote || !$activeSpace) return;

    const newContentString = JSON.stringify(newContent);
    
    try {
      // Pass activeNote.name to the backend
      await updateNoteContent($activeSpace, activeNote.name, {
        content: newContentString,
      });
      console.log("Note content saved successfully.");
    } catch (e) {
      console.error("Failed to save note content:", e);
    }
  }

  $effect(async () => {
    const currentNote = activeNote;
    const currentSpaceName = $activeSpace;

    if (currentNote && currentSpaceName) {
      isLoadingContent = true;
      try {
        // Pass currentNote.name to the backend
        const markdownString = await getNoteContent(
          currentSpaceName,
          currentNote.name,
        );
      } catch (e) {
        console.error("Failed to load note content:", e);
        displayedContent = null;
      } finally {
        isLoadingContent = false;
      }
    } else {
      displayedContent = null;
    }
  });
</script>

<section class="bg-black h-full">
  {#if isLoadingContent}
    <div class="flex items-center justify-center h-full">
      <p class="text-white">Loading Content...</p>
    </div>
  {:else if activeNote}
    <div
      id="editorjsHolder"
      class="h-screen bg-black overflow-y-auto mt-xs rounded-md"
    >
      <EditorSpace
        noteName={activeNote.name}
        initialContent={displayedContent}
        on:content-change={handleContentChange}
      />
    </div>
  {:else}
    <NoNotesInSpace />
  {/if}
</section>