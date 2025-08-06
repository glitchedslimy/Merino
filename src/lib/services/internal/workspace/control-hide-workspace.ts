/**
 * These are sepparate as the control for opening and closing
 * the "My Space" is two sepparate buttons on parts of the app.
 */

import { showWorkspace } from "@stores/workspace-store";

/**
 * Handles the closing of the "My Space" part of the SideBar
 */
export function handleCloseMySpace() {
    showWorkspace.set(false);
};

/**
 * Handles the opening of the "My Space" part of the SideBar
 */
export function handleOpenMySpace() {
    showWorkspace.set(true)
}