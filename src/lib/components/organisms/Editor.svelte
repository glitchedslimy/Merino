<script lang="ts">
  import { Editor as EditorComponent, Utilsbar } from "@molecules";
  import {
    activeSpaceName,
    openNotes,
    activeNoteName, // Changed to activeNoteName
  } from "@stores/workspace-store";
  import { convertMarkdownToJson } from "../../utils/editor-converter";
  import NoNotesInSpace from "@components/molecules/NoNotesInSpace.svelte";
  import {
    getNoteContent,
    updateNoteContent,
  } from "@services/internal/api/tauri-commands";
  import type { OutputData } from "@editorjs/editorjs";
  import NotesTab from "@components/molecules/NotesTab.svelte";

  // The active note is now derived from the activeNoteName
  let activeNote = $derived($openNotes.find(note => note.name === $activeNoteName));
  let displayedContent = $state<OutputData | null>(null);
  let isLoadingContent = $state(false);

  // Function to handle content changes and save the note
  async function handleContentChange(event: CustomEvent<OutputData>) {
    const newContent = event.detail;
    if (!activeNote || !$activeSpaceName) return;

    const newContentString = JSON.stringify(newContent);
    
    try {
      // Pass activeNote.name to the backend
      await updateNoteContent($activeSpaceName, activeNote.name, {
        content: newContentString,
      });
      console.log("Note content saved successfully.");
    } catch (e) {
      console.error("Failed to save note content:", e);
    }
  }

  $effect(async () => {
    const currentNote = activeNote;
    const currentSpaceName = $activeSpaceName;

    if (currentNote && currentSpaceName) {
      isLoadingContent = true;
      try {
        // Pass currentNote.name to the backend
        const markdownString = await getNoteContent(
          currentSpaceName,
          currentNote.name,
        );
        const jsonContent = convertMarkdownToJson(markdownString);
        displayedContent = jsonContent;
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

<section class="">
  <NotesTab />
  <Utilsbar />
  {#if isLoadingContent}
    <div class="flex items-center justify-center h-full">
      <p class="text-white">Loading Content...</p>
    </div>
  {:else if activeNote}
    <div
      id="editorjsHolder"
      class="h-screen bg-black overflow-y-auto mt-xs rounded-md"
    >
      <EditorComponent
        noteName={activeNote.name}
        initialContent={displayedContent}
        on:content-change={handleContentChange}
      />
    </div>
  {:else}
    <NoNotesInSpace />
  {/if}
</section>