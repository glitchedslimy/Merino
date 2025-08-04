import EditorJS, { type ConversionConfig, type I18nConfig, type ToolConfig, type EditorConfig } from "@editorjs/editorjs";
import Header from "@editorjs/header";
import Paragraph from "@editorjs/paragraph";
import List from '@editorjs/list';
import Code from '@editorjs/code';

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
    holder: 'editorjs',
    tools: {
        paragraph: {
            // @ts-ignore due to certain incompatibility although it works
            class: Paragraph,
            inlineToolbar: true,
        },
        header: {
            // @ts-ignore due to certain incompatibility although it works
            class: Header,
            inlineToolbar: true,
            config: {
                placeholder: 'Heading 1',
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
export function initEditor(editor: EditorJS | null) {
    editor = new EditorJS(editorConfig);
    return editor;
}