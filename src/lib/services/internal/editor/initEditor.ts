import EditorJS, { type ConversionConfig, type I18nConfig, type ToolConfig, type EditorConfig } from "@editorjs/editorjs";
import Header from "@editorjs/header";
import List from '@editorjs/list';
import Code from '@editorjs/code';
import DragDrop from "editorjs-drag-drop";
import CustomParagraphTool from "./plugins/pharagraph-conversion";
import type { OutputData } from "@editorjs/editorjs";
import { debounce } from "../../../utils/debounce";
import { invoke } from "@tauri-apps/api/core";
import { convertJsonToMarkdown } from "../../../utils/editor-converter";
import { addToast } from "@stores/toast-store";
/**
 * Conjuctions of interfaces for realizing the config of EditorJS in
 * a sepparate way from the intialization
 * @extends ConversionConfig - Controls the import / export functionality
 * @extends I18nConfig - Controls the Internationalization configuration
 * @extends ToolConfig - The tool config to include tools into EditorJS
 * @extends EditorConfig - Main config for Editorjs
 */
interface EditorJSConfig extends ConversionConfig, I18nConfig, ToolConfig, EditorConfig {}

/**
 * Main configuration for EditorJS
 * @interface EditorJSConfig
 * @example const editorConfig: EditorJSConfig = {
 *  placeholder: "placeholder",
 *  holder: 'editorjs', // What ID of the element has EditorJS included
 *  tools: {
 *    paragraph: {
 *       class: Paragraph,
 *       inlineToolbar: true
 *   }
 *  }
 * }
 */
const editorConfig: EditorJSConfig = {
    placeholder: "Escribe aquí lo que pienses hoy...",
    tools: {
        paragraph: {
            // @ts-ignore due to certain incompatibility although it works
            class: CustomParagraphTool,
            inlineToolbar: true,
        },
        header: {
            // @ts-ignore due to certain incompatibility although it works
            class: Header,
            inlineToolbar: true,
            config: {
                placeholder: 'Mi titulo increible',
                levels: [1, 2, 3, 4, 5, 6],
                defaultLevel: 1
            }
        },
        list: {
            // @ts-ignore due to certain incompatibility although it works
            class: List,
            inlineToolbar: true,
            config: {
                defaultStyle: "unordered"
            }
        },
        code: {
            class: Code,
            config: {
                placeholder: "Your code here"
            }
        }
    },
    i18n: {

    }
}

/**
 * Initializes the editor
 * @param editor - EditorJS Interface
 */
export function initEditor(holderId: string, activeSpaceName: string, noteName: string, initialContent: OutputData | null) {
    console.log(activeSpaceName, noteName, initialContent)
    const debounceSave = debounce(async (contentData: OutputData | undefined) => {
        if (!activeSpaceName || !noteName) {
            console.error("Active space or note name is not defined, can't save")
            return;
        }

        try {
            console.log("debounced content", contentData)
            const markdownString = convertJsonToMarkdown(contentData)
            const encoder = new TextEncoder();
            const contentBytes = Array.from(encoder.encode(markdownString));
            await invoke('save_note_content', {
                spaceName: activeSpaceName,
                noteName: noteName,
                content: contentBytes
            });
        } catch(e) {
            console.error('Failed to save note:', e)
        }
    }, 500)

    let editor = new EditorJS({
        ...editorConfig,
        data: initialContent || { blocks: [] },
        holder: holderId,
        onChange: async () => {
            const data = await editor?.save();
            debounceSave(data)
        },
        onReady: () => {
            new DragDrop(editor)
        }
    });
    return editor;
}