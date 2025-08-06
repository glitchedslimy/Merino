/**
 * Debounces a function call.
 * @param func The function to debounce.
 * @param delay The delay in milliseconds.
 */
export function debounce<T extends any[]>(func: (...args: T) => void, delay: number): (...args: T) => void {
    let timeoutId: number | undefined;

    return (...args: T) => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            func(...args);
        }, delay);
    };
}