<script lang="ts">
    import { Editor, Utilsbar } from "@molecules";
    import { activeNote, activeSpaceName } from "@stores/workspace-store";
    import { convertMarkdownToJson } from "../../utils/editor-converter";
    import { invoke } from "@tauri-apps/api/core";
    
    // Use a local state variable for the editor's content
    let noteContent = $state<OutputData | null>(null);
    let isLoadingContent = $state(false);

    // The effect now watches for changes in the active note's NAME
    // and updates the local state, breaking the reactive loop.
    $effect(async () => {
        const currentNote = $activeNote;
        const currentSpaceName = $activeSpaceName;

        if (currentNote && currentSpaceName) {
            // Check to see if the note name has actually changed
            // This prevents the effect from running if the content is what changed
            isLoadingContent = true;
            try {
                const markdownString = await invoke('load_note_content', {
                    spaceName: currentSpaceName,
                    noteName: currentNote.name,
                });
                
                const jsonContent = convertMarkdownToJson(markdownString);
                
                // Update the local state variable, not the global store
                noteContent = jsonContent;
                
                console.log("Note content loaded:", jsonContent);
            } catch (e) {
                console.error('Failed to load note content:', e);
                noteContent = null;
            } finally {
                isLoadingContent = false;
            }
        } else {
            // Clear the content if no note is active
            noteContent = null;
        }
    });
</script>

<section class="">
    <Utilsbar />
    {#if isLoadingContent}
        <div class="flex items-center justify-center h-full">
            <p class="text-white">Cargando contenido...</p>
        </div>
    {:else if $activeNote && noteContent}
        <Editor noteName={$activeNote.name} initialContent={noteContent} />
    {:else if $activeNote}
        <div class="flex items-center justify-center h-full">
            <p class="text-white">Note has no content yet.</p>
        </div>
    {:else}
        <div class="flex items-center justify-center h-full">
            <p class="text-white">Selecciona una nota para comenzar a editar.</p>
        </div>
    {/if}
</section>