<script lang="ts">
  import { slide } from "svelte/transition";
  import { onDestroy, onMount } from "svelte";
  import {
    activeNoteName,
    opennedNotes,
    activeNoteFolder,
  } from "../../lib/stores/workspace/notes-store";
  import { closeNote } from "../../lib/actions/editor/notes-buffer";
  import { useHorizontalScroll } from "../../lib/useHooks/horizontal-scroll";
  
  let draggedNote: { name: string; folder: string | null } | null =
    $state(null);
  let dropTargetNote: { name: string; folder: string | null } | null =
    $state(null);
  let isDragging = $state(false);
  let tabContainer: HTMLDivElement;

  let showLeftFade = $state(false);
  let showRightFade = $state(false);

  $effect(() => {
    $opennedNotes;
    updateFadeEffects();
  });

  // MODIFIED: Use the note object to select the note
  function selectTab(note: { name: string; folder: string | null }) {
    activeNoteName.set(note.name);
    activeNoteFolder.set(note.folder);
  }

  function handleMouseDown(
    event: MouseEvent,
    note: { name: string; folder: string | null },
  ) {
    if (event.button === 0) {
      draggedNote = note;
    } else if (event.button === 1) {
      event.preventDefault();
      closeNote(note.name, note.folder);
    }
  }

  function handleDragStart(
    event: DragEvent,
    note: { name: string; folder: string | null },
  ) {
    if (event.dataTransfer) {
      event.dataTransfer.setData("text/plain", `${note.name}:${note.folder}`);
    }
    draggedNote = note;
    isDragging = true;
  }

  function handleDragEnter(note: { name: string; folder: string | null }) {
    if (
      note.name !== draggedNote?.name ||
      note.folder !== draggedNote?.folder
    ) {
      dropTargetNote = note;
    }
  }

  function handleDragLeave(note: { name: string; folder: string | null }) {
    if (
      note.name === dropTargetNote?.name &&
      note.folder === dropTargetNote?.folder
    ) {
      dropTargetNote = null;
    }
  }

  function handleDragEnd() {
    if (draggedNote && dropTargetNote) {
      opennedNotes.update((currentNotes) => {
        const draggedIndex = currentNotes.findIndex(
          (n) =>
            n.name === draggedNote?.name && n.folder === draggedNote?.folder,
        );
        const dropIndex = currentNotes.findIndex(
          (n) =>
            n.name === dropTargetNote?.name &&
            n.folder === dropTargetNote?.folder,
        );

        if (draggedIndex !== -1 && dropIndex !== -1) {
          const draggedNoteObject = currentNotes.splice(draggedIndex, 1)[0];
          currentNotes.splice(dropIndex, 0, draggedNoteObject);
        }

        return currentNotes;
      });
    }
    draggedNote = null;
    dropTargetNote = null;
    isDragging = false;
  }

  function handleMouseUp() {
    draggedNote = null;
  }

  function updateFadeEffects() {
    if (tabContainer) {
      const isOverflowing = tabContainer.scrollWidth > tabContainer.clientWidth;
      if (!isOverflowing) {
        showLeftFade = false;
        showRightFade = false;
        return;
      }

      const isAtBeginning = tabContainer.scrollLeft <= 0;
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
    updateFadeEffects();
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
    {#each $opennedNotes as note (note.name + note.folder)}
      <div
        class:text-brand-primary={$activeNoteName === note.name &&
          $activeNoteFolder === note.folder}
        class:bg-black-200={$activeNoteName === note.name &&
          $activeNoteFolder === note.folder}
        onclick={() => selectTab(note)}
        onmousedown={(event) => handleMouseDown(event, note)}
        ondragstart={(event) => handleDragStart(event, note)}
        ondragenter={() => handleDragEnter(note)}
        ondragleave={() => handleDragLeave(note)}
        ondragover={(e) => e.preventDefault()}
        onmouseup={handleMouseUp}
        draggable="true"
        class="flex flex-1 flex-shrink-0 min-w-[12rem] ml-xs bg-black px-sm py-xs rounded-md justify-between relative"
        class:dragging={note.name === draggedNote?.name &&
          note.folder === draggedNote?.folder}
        class:is-drag-over={note.name === dropTargetNote?.name &&
          note.folder === dropTargetNote?.folder}
        transition:slide={{ duration: 50, axis: "x" }}
      >
        <p class="truncate pointer-events-none">{note.name}</p>
        <button
          class="pointer-events-none"
          onclick={() => closeNote(note.name, note.folder)}
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
    scrollbar-width: none;
    -ms-overflow-style: none;
  }

  .tab-container::-webkit-scrollbar {
    display: none;
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
