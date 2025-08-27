<script lang="ts">
    import {
        activeNoteFolder,
        activeNoteName,
    } from "../../lib/stores/workspace/notes-store";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";

    function getPathSegments(path: string): string[] {
        const normalizedPath = path.replace(/\\/g, "/");
        return normalizedPath.split("/").filter(Boolean);
    }
</script>

<div class="w-3xl">
    {#if $activeNoteName}
        <p class="flex gap-x-xs items-center justify-center">
            <span class="text-black-300">{$activeSpace}</span>
            <span class="text-black-300">/</span>
            {#if $activeNoteFolder}
                {#each getPathSegments($activeNoteFolder) as segment, i}
                    <span class="text-black-300">{segment}</span>
                    {#if i < getPathSegments($activeNoteFolder).length - 1}
                        <span class="text-black-300">/</span>
                    {/if}
                {/each}
            {/if}
            <span class="text-black-300">/</span>
            <span class="text-ellipsis whitespace-nowrap overflow-hidden"
                >{$activeNoteName}</span
            >
        </p>
    {:else}
        <p class="items-center justify-center flex">Open a note to start</p>
    {/if}
</div>
