import EditorJS from "@editorjs/editorjs"
import { readOnly } from '../../stores/editor/readOnlyState'
import { get } from "svelte/store"
/**
 * Controls the Read Only mode of Editorjs inside the app
 * @param editor - EditorJS interface, pass the editor to control it
 */
export function changeToReadOnlyMode(editor: EditorJS) {
        readOnly.update(currentValue => !currentValue) // Only for the icons to change
        const isInEditMode = get(readOnly)
        console.log(isInEditMode)
        if(editor && editor.readOnly) {
            editor.readOnly.toggle();
        }
}