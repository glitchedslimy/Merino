<script lang="ts">
    import { closeNote } from "@services/internal/editor/notes-buffer";
    import { openNotes, activeNoteId } from "@stores/workspace-store";
    import { slide } from "svelte/transition";
    import { useHorizontalScroll } from "../../utils/horizontal-scroll";
    import { onDestroy, onMount } from "svelte";

    let draggedNoteId: string | null = $state(null);
    let dropTargetNoteId: string | null = $state(null);
    let isDragging = $state(false);
    let tabContainer: HTMLDivElement;

    // New state variables to control fade visibility
    let showLeftFade = $state(false);
    let showRightFade = $state(false);

    $effect(() => {
        $openNotes;
        updateFadeEffects();
    });

    function selectTab(noteId: string) {
        activeNoteId.set(noteId);
    }

    function handleMouseDown(event: MouseEvent, noteId: string) {
        if (event.button === 0) {
            draggedNoteId = noteId;
        } else if (event.button === 1) {
            event.preventDefault();
            closeNote(noteId);
        }
    }

    function handleDragStart(event: DragEvent, noteId: string) {
        if (event.dataTransfer) {
            event.dataTransfer.setData("text/plain", noteId);
        }
        draggedNoteId = noteId;
        isDragging = true;
        console.log(isDragging)
    }

    function handleDragEnter(noteId: string) {
        if (noteId !== draggedNoteId) {
            dropTargetNoteId = noteId;
        }
    }

    function handleDragLeave(noteId: string) {
        if (noteId === dropTargetNoteId) {
            dropTargetNoteId = null;
        }
    }

    function handleDragEnd() {
        if (
            draggedNoteId &&
            dropTargetNoteId &&
            draggedNoteId !== dropTargetNoteId
        ) {
            openNotes.update((currentNotes) => {
                const draggedIndex = currentNotes.findIndex(
                    (n) => n.id === draggedNoteId,
                );
                const dropIndex = currentNotes.findIndex(
                    (n) => n.id === dropTargetNoteId,
                );

                if (draggedIndex !== -1 && dropIndex !== -1) {
                    const draggedNote = currentNotes.splice(draggedIndex, 1)[0];
                    currentNotes.splice(dropIndex, 0, draggedNote);
                }

                return currentNotes;
            });
        }
        draggedNoteId = null;
        dropTargetNoteId = null;
        isDragging = false;
    }

    function handleMouseUp() {
        draggedNoteId = null;
    }

    function updateFadeEffects() {
        if (tabContainer) {
            // Check if there is overflow
            const isOverflowing =
                tabContainer.scrollWidth > tabContainer.clientWidth;
            if (!isOverflowing) {
                showLeftFade = false;
                showRightFade = false;
                return;
            }

            // Check if scroll is at the beginning
            const isAtBeginning = tabContainer.scrollLeft <= 0;
            // Check if scroll is at the end with a 1px tolerance
            const isAtEnd =
                Math.abs(
                    tabContainer.scrollLeft +
                        tabContainer.clientWidth -
                        tabContainer.scrollWidth,
                ) <= 1;

            showLeftFade = !isAtBeginning;
            showRightFade = !isAtEnd;
        }
    }

    onMount(() => {
        window.addEventListener("resize", updateFadeEffects);
        updateFadeEffects(); // Initial check
    });

    onDestroy(() => {
        window.removeEventListener("resize", updateFadeEffects);
    });
</script>

<div
    class="scroll-fade-wrapper"
    class:show-left-fade={showLeftFade}
    class:show-right-fade={showRightFade}
>
    <div
        class="tab-container overflow-x-auto flex flex-nowrap mb-xs rounded-sm w-full items-center"
        ondragend={handleDragEnd}
        onscroll={updateFadeEffects}
        use:useHorizontalScroll
        bind:this={tabContainer}
    >
        {#each $openNotes as note (note.id)}
            <div
                class:text-brand-primary={$activeNoteId === note.id}
                class:bg-black-200={$activeNoteId === note.id}
                onclick={() => selectTab(note.id)}
                onmousedown={(event) => handleMouseDown(event, note.id)}
                ondragstart={(event) => handleDragStart(event, note.id)}
                ondragenter={() => handleDragEnter(note.id)}
                ondragleave={() => handleDragLeave(note.id)}
                ondragover={(e) => e.preventDefault()}
                onmouseup={handleMouseUp}
                draggable="true"
                class="flex flex-1 flex-shrink-0 min-w-[12rem] ml-xs bg-black px-sm py-xs rounded-md justify-between relative"
                class:dragging={note.id === draggedNoteId}
                class:is-drag-over={note.id === dropTargetNoteId}
                transition:slide={{ duration: 50, axis: "x" }}
            >
                <p class="truncate pointer-events-none">{note.name}</p>
                <button
                    class="pointer-events-none"
                    onclick={() => closeNote(note.id)}
                    class:pointer-events-none={isDragging}
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>
        {/each}
    </div>
</div>

<style>
    .dragging {
        opacity: 0.5;
    }

    .is-drag-over::before {
        content: "";
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        width: 2px;
        background-color: var(--color-brand-primary);
        border-radius: 3px;
        z-index: 10;
        transform: translateX(-50%);
    }

    .scroll-fade-wrapper {
        position: relative;
    }

    .tab-container {
        scrollbar-width: none; /* Firefox */
        -ms-overflow-style: none; /* Internet Explorer and Edge */
    }

    .tab-container::-webkit-scrollbar {
        display: none; /* Chrome, Safari, Opera */
    }

    .scroll-fade-wrapper::before,
    .scroll-fade-wrapper::after {
        content: "";
        position: absolute;
        top: 0;
        bottom: 0;
        width: 2rem;
        z-index: 1;
        pointer-events: none;
        transition: opacity 0.3s ease-in-out;
        opacity: 0;
    }

    .scroll-fade-wrapper.show-left-fade::before {
        left: 0;
        background: linear-gradient(
            to right,
            var(--background-color, #18191a) 0%,
            rgba(24, 25, 26, 0) 100%
        );
        opacity: 1;
    }

    .scroll-fade-wrapper.show-right-fade::after {
        right: 0;
        background: linear-gradient(
            to left,
            var(--background-color, #18191a) 0%,
            rgba(24, 25, 26, 0) 100%
        );
        opacity: 1;
    }
</style>
