<script>
    import Settings from "@components/molecules/Settings.svelte";
    import { openSettings } from "../../lib/stores/settings/settings";
    import { fade } from "svelte/transition";

</script>
{#if $openSettings}
<section class="bg-black/50 absolute top-8 z-50 h-[95%] w-full flex justify-center" transition:fade={{ duration: 200 }}>
    <Settings />
</section>
{/if}