import { invoke } from "@tauri-apps/api/core";

/**
 * Interface of the Space
 */
interface Space {
    name: string;
}

/**
 * Fetches the space from the Rust function list_spaces_cmd
 * @param currentSpace 
 * @param availableSpaces 
 */
export async function fetchSpace(currentSpace: Space, availableSpaces: Space[]): Promise<void> {
    let promiseSpaces: Space[] = await invoke('list_spaces_cmd');
    availableSpaces = promiseSpaces;

    const savedSpaceName = localStorage.getItem('currentSpaceName');
    if(savedSpaceName) {
        const foundSpace = promiseSpaces.find((s: Space) => s.name === savedSpaceName);
        if(foundSpace) currentSpace = foundSpace;
    } else if(promiseSpaces.length > 0) {
        currentSpace = promiseSpaces[0]
    }
}

/**
 * Handles the space selected for the dropdown
 * @param currentSpace 
 * @param selectedSpace 
 */
export function handleSpaceSelected(currentSpace: Space, selectedSpace: Space) {
    currentSpace = selectedSpace;
    localStorage.setItem('currentSpaceName', selectedSpace.name);
}