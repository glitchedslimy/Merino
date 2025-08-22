// src/actions/useHorizontalScroll.ts

export function useHorizontalScroll(node: HTMLElement) {
    const handleWheel = (event: WheelEvent) => {
        // Only trigger on vertical mouse wheel scroll
        if (Math.abs(event.deltaY) > Math.abs(event.deltaX)) {
            // Prevent the default vertical scrolling behavior
            event.preventDefault();
            // Scroll the element horizontally
            node.scrollLeft += event.deltaY;
        }
    };

    node.addEventListener('wheel', handleWheel);

    return {
        destroy() {
            node.removeEventListener('wheel', handleWheel);
        }
    };
}