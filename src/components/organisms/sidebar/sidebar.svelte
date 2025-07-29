<script lang="ts">
    import Myspace from "../../molecules/sidebar/myspace.svelte";
    import Privatespace from "../../molecules/sidebar/privatespace.svelte";
    let { closeMySpace = $bindable(), handleCloseMySpace, className} = $props();
</script>

{#if !closeMySpace}
<section class="bg-black px-8 py-8 h-screen flex flex-col space-y-5">
        <Myspace bind:closeMySpace={closeMySpace} handleMySpaceClose={handleCloseMySpace}/>
        <Privatespace />
</section>
{/if}