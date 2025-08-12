// src/actions/ellipsis.ts
import type { Action } from "svelte/action";

export const ellipsisTooltip: Action<HTMLElement, { onShow: (text: string, x: number, y: number) => void; onHide: () => void }> = (node, params) => {
    const isEllipsisActive = () => node.offsetWidth < node.scrollWidth;
    let isActive = false;

    const showTooltip = (event: MouseEvent) => {
        if (isEllipsisActive()) {
            isActive = true;
            params?.onShow(node.textContent || "", event.clientX + 10, event.clientY + 10);
        }
    };

    const hideTooltip = () => {
        if (isActive) {
            isActive = false;
            params?.onHide();
        }
    };

    node.addEventListener("mouseenter", showTooltip);
    node.addEventListener("mouseleave", hideTooltip);
    node.addEventListener("mousemove", (event) => {
        if (isActive) {
            params?.onShow(node.textContent || "", event.clientX + 10, event.clientY + 10);
        }
    });

    return {
        destroy() {
            node.removeEventListener("mouseenter", showTooltip);
            node.removeEventListener("mouseleave", hideTooltip);
        },
    };
};