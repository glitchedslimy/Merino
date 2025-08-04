/**
 * These are sepparate as the control for opening and closing
 * the "My Space" is two sepparate buttons on parts of the app.
 */

import { showMySpace } from "../../stores/mySpace/mySpaceState";

/**
 * Handles the closing of the "My Space" part of the SideBar
 * @param closeMySpace 
 */
export function handleCloseMySpace() {
    showMySpace.set(false);
};

/**
 * Handles the opening of the "My Space" part of the SideBar
 * @param closeMySpace 
 */
export function handleOpenMySpace() {
    showMySpace.set(true)
}