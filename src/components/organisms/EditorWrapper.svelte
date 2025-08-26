<script lang="ts">
  import type { OutputData } from "@editorjs/editorjs";
  import {
    activeNoteName,
    opennedNotes,
    activeNoteFolder,
  } from "../../lib/stores/workspace/notes-store";
  import { activeSpace } from "../../lib/stores/workspace/spaces-store";
  import EditorSpace from "./EditorSpace.svelte";
  import { updateNoteContent } from "../../lib/api/tauri/update/notes-api-update";
  import { getNoteContent } from "../../lib/api/tauri/get/notes-api-get";
  import NoNotesInSpace from "./NoNotesInSpace.svelte";

  let activeNote = $derived(
    $opennedNotes.find(
      (note) =>
        note.name === $activeNoteName && note.folder === $activeNoteFolder,
    ),
  );
  let displayedContent = $state<OutputData | null>(null);
  let isLoadingContent = $state(false);

  async function handleContentChange(event: CustomEvent<OutputData>) {
    const newContent = event.detail;
    if (!activeNote || !$activeSpace) return;

    const newContentString = JSON.stringify(newContent);

    try {
      await updateNoteContent(
        $activeSpace,
        activeNote.name,
        {
          content: newContentString,
        },
        activeNote.folder,
      );
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
        const noteData = await getNoteContent(
          currentSpaceName,
          currentNote.name,
          currentNote.folder,
        );
        displayedContent = JSON.parse(noteData.content);
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
        on:content-change={handleContentChange}
        noteFolder={activeNote.folder}
      />
    </div>
  {:else}
    <NoNotesInSpace />
  {/if}
</section>
