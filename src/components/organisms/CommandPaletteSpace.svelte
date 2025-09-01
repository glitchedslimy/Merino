<script>
    import CommandPalette from "@components/molecules/CommandPalette.svelte";
    import { showCommandPalette } from "../../lib/stores/commandpalette/commandpalette";

</script>
{#if $showCommandPalette}
<section class="bg-black/50 absolute top-8 z-50 h-[90%] w-full flex justify-center ">
    <CommandPalette />
</section>
{/if}
