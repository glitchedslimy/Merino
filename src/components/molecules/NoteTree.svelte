<script lang="ts">
  import { ellipsisTooltip } from "../../lib/useHooks/ellipsisTooltip";
  import Button from "@components/atoms/Button.svelte";
  import Icon from "@components/atoms/Icon.svelte";
  import Input from "@components/atoms/Input.svelte";
  import Tooltip from "@components/atoms/Tooltip.svelte";
  import { get } from "svelte/store";
  import {
    createNote,
    handleHideTooltip,
    handleShowTooltip,
    loadNotesInSpace,
    renameNote,
    selectNote,
    showNoteContextmenu,
  } from "../../lib/actions/workspace/notes";
  import {
    moveNoteToFolder,
    moveFolderToFolder,
    showFolderContextmenu,
    renameFolder,
  } from "../../lib/actions/workspace/folders";
  import { contextMenuVisible } from "../../lib/stores/contextmenu/contextmenu-store";
  import {
    activeNoteFolder,
    activeNoteName,
    currentRightClickedNote,
    inputRenameNoteElement,
    renamedNote,
  } from "../../lib/stores/workspace/notes-store";
  import { activeSpace } from "../../lib/stores/workspace/spaces-store";
  import {
    showTooltip,
    tooltipText,
    tooltipX,
    tooltipY,
  } from "../../lib/stores/tooltip/tooltip-store";
  import type { Note } from "../../lib/types/notes";
  import NoteTree from "./NoteTree.svelte";
  import { closeNote } from "../../lib/actions/editor/notes-buffer";
  import {
    inputRenameFolderElement,
    renamedFolder,
  } from "../../lib/stores/workspace/folders-store";
  import { slide } from "svelte/transition";

  let { content, name, path = "" } = $props();

  let isFolderOpen = $state(false);
  let dragCount = $state(0);
  const isDraggedOver = $derived(dragCount > 0);

  function toggleFolder() {
    if ($renamedFolder) {
      return false;
    }
    isFolderOpen = !isFolderOpen;
  }

  function handleDragStart(e: DragEvent, note: Note) {
    if (
      $renamedNote?.name === note.name &&
      $renamedFolder?.name === note.folder
    ) {
      e.preventDefault();
      return;
    }
    e.dataTransfer?.setData("text/name", note.name);
    e.dataTransfer?.setData("text/folder", note.folder || "");
    e.dataTransfer?.setData("text/isFolder", "false");
  }

  function handleFolderDragStart(e: DragEvent) {
    if ($renamedNote?.name === name || $renamedFolder?.name === name) {
      e.preventDefault();
      return;
    }
    e.dataTransfer?.setData("text/isFolder", "true");

    // The `path` prop in your component is the full path (e.g., "Untitled 2").
    // The `name` prop is just the name (e.g., "Untitled 2").
    const parentPath = getParentPath(path);

    // Set the data with only the parent path and the folder name separately.
    e.dataTransfer?.setData("text/folderPath", parentPath || "");
    e.dataTransfer?.setData("text/folderName", name);
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function handleDragEnter() {
    dragCount++;
  }

  function handleDragLeave() {
    dragCount--;
  }

  function getParentPath(fullPath: string): string | null {
    if (!fullPath || fullPath.indexOf("/") === -1) {
      return null;
    }
    const parts = fullPath.split("/");
    parts.pop();
    return parts.join("/");
  }

  async function handleDrop(event: DragEvent, newFolder: string | null) {
    event.preventDefault();
    event.stopPropagation();
    dragCount = 0;
    try {
      const isFolder = event.dataTransfer?.getData("text/isFolder") === "true";
      if (isFolder) {
        const draggedFolderPath =
          event.dataTransfer?.getData("text/folderPath");
        const draggedFolderName =
          event.dataTransfer?.getData("text/folderName");

        if (draggedFolderName && draggedFolderPath !== newFolder) {
          await moveFolderToFolder(
            get(activeSpace),
            draggedFolderName,
            draggedFolderPath,
            newFolder ?? null,
          );
        }
      } else {
        const draggedNoteName = event.dataTransfer?.getData("text/name");
        const draggedFolder = event.dataTransfer?.getData("text/folder");
        const oldFolderValue = draggedFolder === "" ? null : draggedFolder;

        if (
          draggedNoteName &&
          get(activeSpace) &&
          oldFolderValue !== newFolder
        ) {
          await moveNoteToFolder(
            get(activeSpace),
            draggedNoteName,
            oldFolderValue,
            newFolder ?? null,
          );
        }
        closeNote(draggedNoteName ?? "", newFolder);
      }
      loadNotesInSpace($activeSpace);
    } catch (e) {
      console.error(e);
    }
  }
</script>

{#if name !== ""}
  <li class="flex flex-col gap-y-1">
    <div class="flex items-center">
      <Button
        intent="notes"
        onClick={toggleFolder}
        ondragover={handleDragOver}
        ondragenter={handleDragEnter}
        ondragleave={handleDragLeave}
        ondrop={(e) => handleDrop(e, path)}
        ondragstart={(e) => handleFolderDragStart(e)}
        oncontextmenu={(e) => showFolderContextmenu(e, name, path)}
        draggable={true}
        class={`${isDraggedOver ? "bg-brand-primary-light" : ""} flex-grow flex justify-between items-center`}
      >
        <div class="flex items-center gap-x-xs">
          {#if isFolderOpen}
            <Icon iconName="folder-open" width="20" />
          {:else}
            <Icon iconName="folder" width="20" />
          {/if}
          {#if $renamedFolder?.name === name && $renamedFolder?.path === path}
            <Input
              bind:inputElement={$inputRenameFolderElement}
              inputType="text"
              value={name}
              class="flex-grow"
              onKeydown={(e) => {
                if (e.key === "Enter") {
                  renameFolder(e.currentTarget?.value);
                } else if (e.key === "Escape") {
                  renamedFolder.set(null);
                }
              }}
              onblur={(e) => {
                renameFolder(e.currentTarget?.value);
              }}
            />
          {:else}
            <span
              use:ellipsisTooltip={{
                onShow: handleShowTooltip,
                onHide: handleHideTooltip,
              }}
              class="text-ellipsis whitespace-nowrap overflow-hidden max-w-2xl"
            >
              {name}
            </span>
          {/if}
        </div>
        <Icon
          iconName="chevron-down"
          width="16"
          class={`transition-transform ${isFolderOpen ? "rotate-180" : ""}`}
        />
      </Button>
    </div>
    {#if isFolderOpen}
      <div class="ml-sm" transition:slide={{ duration: 50, axis: "y" }}>
        {#if content && content.folders}
          {#each Object.entries(content.folders) as [folderName, folderContent]}
            <NoteTree
              name={folderName}
              content={folderContent}
              path={`${path}/${folderName}`}
            />
          {/each}
        {/if}
        {#each content.notes as note}
          <div
            class="flex items-center"
            oncontextmenu={(e) => showNoteContextmenu(e, note)}
            role="listitem"
          >
            <Button
              intent="notes"
              onClick={() => {
                selectNote(note);
              }}
              class={`${$activeNoteName === note.name && $activeNoteFolder === note.folder ? "text-brand-primary-light bg-black-200" : ""} ${get(currentRightClickedNote)?.name === note.name && get(currentRightClickedNote)?.folder === note.folder ? "border-[1.5px] border-brand-primary-dark" : ""} flex-grow flex items-center gap-x-xs`}
              draggable={!(
                $renamedNote?.name === note.name &&
                $renamedNote?.folder === note.folder
              )}
              ondragstart={(e) => handleDragStart(e, note)}
            >
              <Icon iconName="note" width="20" />
              {#if $renamedNote?.name === note.name && $renamedNote?.folder === note.folder}
                <Input
                  bind:inputElement={$inputRenameNoteElement}
                  inputType="text"
                  value={note.name}
                  class="flex-grow"
                  onKeydown={(e) => {
                    if (e.key === "Enter") {
                      renameNote(e.currentTarget?.value);
                    } else if (e.key === "Escape") {
                      renamedNote.set(null);
                    }
                  }}
                  onblur={(e) => {
                    renameNote(e.currentTarget?.value);
                  }}
                />
              {:else}
                <span
                  use:ellipsisTooltip={{
                    onShow: handleShowTooltip,
                    onHide: handleHideTooltip,
                  }}
                  class="text-ellipsis whitespace-nowrap overflow-hidden"
                >
                  {note.name}
                </span>
              {/if}
            </Button>
          </div>
        {/each}
      </div>
    {/if}
  </li>
{:else}
  <div
    ondragover={handleDragOver}
    ondragenter={handleDragEnter}
    ondragleave={handleDragLeave}
    ondrop={(e) => handleDrop(e, null)}
    role="list"
    class="h-full"
  >
    <ul>
      {#if content && content.folders}
        {#each Object.entries(content.folders) as [folderName, folderContent]}
          <NoteTree
            name={folderName}
            content={folderContent}
            path={folderName}
          />
        {/each}
      {/if}
      {#each content.notes as note}
        <div
          class="flex items-center"
          oncontextmenu={(e) => showNoteContextmenu(e, note)}
          role="listitem"
        >
          <Button
            intent="notes"
            onClick={() => {
              selectNote(note);
            }}
            class={`${$activeNoteName === note.name && $activeNoteFolder === note.folder ? "text-brand-primary-light bg-black-200" : ""} ${get(currentRightClickedNote)?.name === note.name && get(currentRightClickedNote)?.folder === note.folder ? "border-[1.5px] border-brand-primary-dark" : ""} flex-grow flex items-center gap-x-xs`}
            draggable={!(
              $renamedNote?.name === note.name &&
              $renamedNote?.folder === note.folder
            )}
            ondragstart={(e) => handleDragStart(e, note)}
          >
            <Icon iconName="note" width="20" />
            {#if $renamedNote?.name === note.name && $renamedNote?.folder === note.folder}
              <Input
                bind:inputElement={$inputRenameNoteElement}
                inputType="text"
                value={note.name}
                class="flex-grow"
                onKeydown={(e) => {
                  if (e.key === "Enter") {
                    renameNote(e.currentTarget?.value);
                  } else if (e.key === "Escape") {
                    renamedNote.set(null);
                  }
                }}
                onblur={(e) => {
                  renameNote(e.currentTarget?.value);
                }}
              />
            {:else}
              <span
                use:ellipsisTooltip={{
                  onShow: handleShowTooltip,
                  onHide: handleHideTooltip,
                }}
                class="text-ellipsis whitespace-nowrap overflow-hidden"
              >
                {note.name}
              </span>
            {/if}
          </Button>
        </div>
      {/each}
    </ul>
  </div>
{/if}

{#if $showTooltip && !$contextMenuVisible}
  <Tooltip text={$tooltipText} x={$tooltipX} y={$tooltipY} />
{/if}
