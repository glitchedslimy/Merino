import EditorJS, { type ConversionConfig, type I18nConfig, type ToolConfig, type EditorConfig, type OutputData } from "@editorjs/editorjs";
import CustomParagraphTool from "./plugins/pharagraph-conversion";
import Header from "@editorjs/header";
import DragDrop from "editorjs-drag-drop";
import { debounce } from "./debounce";
import { updateNoteContent } from "../../api/tauri/update/notes-api-update";
import EditorjsList from '@editorjs/list';
import EmbedControl from 'editorjs-embed-control';
import Quote from "@cychann/editorjs-quote";
import Delimiter from "@editorjs/delimiter";
import Attaches from '@editorjs/attaches';
import ColorPicker from "editorjs-color-picker";
import Marker from "@editorjs/marker";
import Alert from 'editorjs-alert';
import Table from '@editorjs/table'
import Strike from '@sotaproject/strikethrough'
import Hotkey from 'editorjs-inline-hotkey'
import Code from '@calumk/editorjs-codecup'
interface EditorJSConfig extends ConversionConfig, I18nConfig, ToolConfig, EditorConfig { }

const editorConfig: EditorJSConfig = {
    placeholder: "Hey! What are we going to do today?",

    tools: {
        code: {
            class: Code,
            inlineToolbar: true
        },
        hotkey: {
            class: Hotkey,
            inlineToolbar: true
        },
        strike: {
            class: Strike,
            inlineToolbar: true
        },
        Table: {
            class: Table,
            inlineToolbar: true
        },
        Marker: {
            class: Marker,
            inlineToolbar: true
        },
        alert: {
            class: Alert,
            inlineToolbar: true
        },
        ColorPicker: {
            class: ColorPicker,
        },
        list: {
            class: EditorjsList,
            inlineToolbar: true
        },
        paragraph: {
            // @ts-ignore due to certain incompatibility although it works
            class: CustomParagraphTool,
            inlineToolbar: true
        },
        delimiter: {
            class: Delimiter,
            inlineToolbar: true
        },
        quote: {
            class: Quote,
            inlineToolbar: true,
        },
        embed: {
            class: EmbedControl,
            config: {
                services: {
                    youtube: true,
                    figma: {
                        regex: /https:\/\/((www\.|embed\.)?)figma\.com\/(file|proto|design|board|slides|deck)\/([a-zA-Z0-9]{22,128})/,
                        embedUrl: 'https://embed.figma.com/<%= remote_id %>?embed-host=your_product_name',
                        html: "<iframe height='300' scrolling='no' frameborder='no' allowtransparency='true' allowfullscreen='true' style='width: 100%;'></iframe>",
                        height: 600,
                        width: 600,
                        id: (groups) => `${groups[2]}/${groups[3]}`
                    },
                    codepen: {
                        regex: /https?:\/\/codepen.io\/([^\/\?\&]*)\/pen\/([^\/\?\&]*)/,
                        embedUrl: 'https://codepen.io/<%= remote_id %>?height=300&theme-id=0&default-tab=css,result&embed-version=2',
                        html: "<iframe height='300' scrolling='no' frameborder='no' allowtransparency='true' allowfullscreen='true' style='width: 100%;'></iframe>",
                        height: 300,
                        width: 600,
                        id: (groups) => groups.join('/embed/')
                    }
                }
            }
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

export function initializeEditor(holderId: string, activeSpaceName: string, noteName: string, initialContent: string | null, noteFolder: string | null = null) {
    const debounceSave = debounce(async (contentData: OutputData | undefined) => {
        if (!activeSpaceName || !noteName) {
            return;
        }

        try {
            const jsonString = JSON.stringify(contentData);
            const encoder = new TextEncoder();
            const contentBytes = Array.from(encoder.encode(jsonString));

            await updateNoteContent(activeSpaceName, noteName, contentBytes, noteFolder)
        } catch (e) {
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
            new DragDrop(editor);

            if (!initialContent) return;

            try {
                let jsonString = initialContent;

                // Now parse JSON safely
                const editorData: OutputData = JSON.parse(jsonString);

                // Render EditorJS content
                await editor.blocks.render(editorData);
            } catch (e) {
                console.error("Failed to load initial content:", e);
            }
        }
    })
    return editor;
}