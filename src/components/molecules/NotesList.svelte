<script lang="ts">
  import {
    handleNoteActionContextMenu,
    loadNotesInSpace,
  } from "../../lib/actions/workspace/notes";
  import {
    currentRightClickedNote,
    notes,
  } from "../../lib/stores/workspace/notes-store";
  import { activeSpace } from "../../lib/stores/workspace/spaces-store";
  import {
    showTooltip,
    tooltipText,
    tooltipX,
    tooltipY,
  } from "../../lib/stores/tooltip/tooltip-store";
  import Tooltip from "@components/atoms/Tooltip.svelte";
  import { contextMenuVisible } from "../../lib/stores/contextmenu/contextmenu-store";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import NoteTree from "./NoteTree.svelte";

  $effect(() => {
    loadNotesInSpace($activeSpace);
    let unlisten: UnlistenFn | undefined;
    (async () => {
      unlisten = await listen("tauri://menu", (event) => {
        if (event.payload === "delete-note" && $currentRightClickedNote) {
          handleNoteActionContextMenu("delete");
        }
      });
    })();
    return () => {
      if (unlisten) unlisten();
    };
  });
  
  let { class: className } = $props();
</script>

<section class={`${className} px-xs overflow-y-auto`}>
  <NoteTree content={$notes} name="" />
</section>

{#if $showTooltip && !$contextMenuVisible}
  <Tooltip text={$tooltipText} x={$tooltipX} y={$tooltipY} />
{/if}