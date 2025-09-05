/**
 * A Svelte action to detect clicks outside of a node.
 * Dispatches a 'click_outside' event when a click occurs outside the element.
 * * @param {HTMLElement} node - The element to detect clicks outside of.
 */
export function handleClickOutside(node: HTMLElement) {
    const handleDocumentClick = (event: MouseEvent) => {
        // Check if the click is outside the node and is not on a menu toggle button
        if (node && !node.contains(event.target as Node)) {
            node.dispatchEvent(new CustomEvent('click_outside'));
        }
    };

    document.addEventListener('click', handleDocumentClick, true);

    return {
        destroy() {
            document.removeEventListener('click', handleDocumentClick, true);
        }
    };
}