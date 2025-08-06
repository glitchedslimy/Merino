/**
 * These are sepparate functions as the functionalities for
 * the settings modal are sepparate from each other.
 */

import { showSettingsModal } from "@stores/sidebar-store";

/**
 * Controls the opening of the settings modal 
 */
export function openSettings() {
    showSettingsModal.set(true);
}

/**
 * Controls the closing of the settings modal
 */
export function closeSettings() {
    showSettingsModal.set(false);
}