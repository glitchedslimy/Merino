<script>
    import NotesSpace from "@components/organisms/NotesSpace.svelte";
    import WorkspaceSpace from "@components/organisms/WorkspaceSpace.svelte";
    import { hideWorkspace } from "../../lib/stores/workspace/workspace-store";
    import { slide } from "svelte/transition";

</script>
{#if !$hideWorkspace}
<div class="bg-black rounded-md p-lg flex flex-col gap-y-lg mr-xs w-[17rem]"
transition:slide={{ duration: 200, axis: "x"}}>
    <WorkspaceSpace />
    <NotesSpace />
</div>
{/if}