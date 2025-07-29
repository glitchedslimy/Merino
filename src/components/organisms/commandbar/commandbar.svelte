<script lang="ts">
    import Button from "../../atoms/button/button.svelte";
    import Icon from "../../atoms/icon/icon.svelte";
    // Accept a 'class' prop from the parent
    let { onOpenSettings, class: className = '' } = $props();
</script>

<section
    class="flex flex-col justify-between px-3 bg-black-100 h-screen ${className}"
>
    <div class="flex flex-col space-y-8 py-7">
        <div>
            <Icon iconName="terminal" />
        </div>
        <div>
            <Icon iconName="search" />
        </div>
    </div>
    <div class="py-14">
        <Button type="button" handleClick={() => onOpenSettings()} icon>
            <Icon iconName="settings" />
        </Button>
    </div>
</section>