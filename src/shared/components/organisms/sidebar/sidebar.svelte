<script lang="ts">
    import { slide } from "svelte/transition";
    import {Myspace, Privatespace} from '@molecules'
    let { className, currentSpaceName, onSpaceSelected, availableSpaces } = $props();
    import { showMySpace } from '@stores/mySpace/mySpaceState'
</script>

{#if $showMySpace}
<section class="bg-black px-8 py-8 h-screen flex flex-col space-y-5" transition:slide={{ duration: 200, axis: "x" }}>
        <Myspace {currentSpaceName} {onSpaceSelected} {availableSpaces}/>
        <Privatespace />
</section>
{/if}