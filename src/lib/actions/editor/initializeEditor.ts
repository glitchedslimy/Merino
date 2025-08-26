import EditorJS, { type ConversionConfig, type I18nConfig, type ToolConfig, type EditorConfig, type OutputData } from "@editorjs/editorjs";
import CustomParagraphTool from "./plugins/pharagraph-conversion";
import Header from "@editorjs/header"
import DragDrop from "editorjs-drag-drop";
import { debounce } from "./debounce";
import { convertJsonToMarkdown } from "./editor-converter";
import { updateNoteContent } from "../../api/tauri/update/notes-api-update";


interface EditorJSConfig extends ConversionConfig, I18nConfig, ToolConfig, EditorConfig {}

const editorConfig: EditorJSConfig = {
    placeholder: "Hey! What are we going to do today?",
    tools: {
        paragraph: {
            // @ts-ignore due to certain incompatibility although it works
            class: CustomParagraphTool,
            inlineToolbar: true
        },
        header: {
            // @ts-ignore due to certain incompatibility although it works
            class: Header,
            inlineToolbar: true,
            config: {
                placeholder: "New Title",
                levels: [1, 2, 3, 4, 5, 6],
                defaultLevel: 1
            }
        }
    }
}

export function initializeEditor(holderId: string, activeSpaceName: string, noteName: string, noteId: string, initialContent: string | null, noteFolder: string | null = null) {
    const debounceSave = debounce(async (contentData: OutputData | undefined) => {
        if(!activeSpaceName || !noteName) {
            return;
        }

        try {
            const markdownString = convertJsonToMarkdown(contentData);
            const encoder = new TextEncoder();
             const contentBytes = Array.from(encoder.encode(markdownString));

            await updateNoteContent(activeSpaceName, noteName, contentBytes, noteFolder)
        } catch(e) {
            console.error("Failed to save note: ", e)
        }
    }, 500)

    let editor = new EditorJS({
        ...editorConfig,
        holder: holderId,
        onChange: async () => {
            const data = await editor?.save();
            debounceSave(data)
        },
        onReady: async () => {
            new DragDrop(editor)
            if(initialContent) {
                await editor.blocks.renderFromHTML(initialContent)
            }
        }
    })
    return editor;
}