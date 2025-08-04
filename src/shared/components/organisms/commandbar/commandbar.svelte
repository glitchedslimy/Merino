<script lang="ts">
    import { fade } from "svelte/transition";
    import {Button, Icon} from '@atoms'
    import { showMySpace } from "@stores/mySpace/mySpaceState";
    import { openSettings } from '@scripts/settings/settings'
</script>

<section
    class="flex flex-col justify-between px-3 bg-black-100 h-screen"
>
    <div class="flex flex-col space-y-8 py-7">
        <div>
            <Icon iconName="terminal" />
        </div>
        <div>
            <Icon iconName="search" />
        </div>
        {#if !$showMySpace}
            <div transition:fade={{ duration: 200 }}>
                <Icon iconName="ai" />
            </div>
            <div transition:fade={{ duration: 200 }}>
                <Icon iconName="templates" />
            </div>
        {/if}
    </div>
    <div class="py-14">
        <Button type="button" handleClick={openSettings} icon>
            <Icon iconName="settings" />
        </Button>
    </div>
</section>