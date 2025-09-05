import EditorJS, { type OutputData } from "@editorjs/editorjs";
import { activeSpace } from "../../stores/workspace/spaces-store";
import { get } from "svelte/store";
import { initializeEditor } from "./initializeEditor";
import { activeNoteName } from "../../stores/workspace/notes-store";

export async function reloadEditor(editorInstance: EditorJS | null, initialContent: OutputData | null, noteName: string, holderId: string) {
    let currentContent: OutputData | null = null;

    if (editorInstance && typeof editorInstance.save === 'function') {
        try {
            currentContent = await editorInstance.save();
        } catch(e) {
            currentContent = initialContent;
        }
    }

    if(editorInstance && typeof editorInstance.destroy === 'function') {
        editorInstance.destroy();
        editorInstance = null;
    }

    if(noteName && get(activeSpace)) {
        editorInstance = initializeEditor(
            holderId,
            get(activeSpace) ?? '',
            noteName,
            get(activeNoteName) ?? '',
            currentContent || initialContent
        )
    }
}