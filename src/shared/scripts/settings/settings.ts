/**
 * These are sepparate functions as the functionalities for
 * the settings modal are sepparate from each other.
 */

import { showSettingsModal } from "../../stores/settingsModal/settingsModalState";

/**
 * Controls the opening of the settings modal
 * @param showSettingsModal 
 */
export function openSettings() {
    console.log('Opening Settings Modal...'); 
    showSettingsModal.set(true);
}

/**
 * Controls the closing of the settings modal
 * @param showSettingsModal 
 */
export function closeSettings() {
    console.log('Closing Settings Modal...');
    showSettingsModal.set(false);
}