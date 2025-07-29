<script lang="ts">
    import Button from "../../atoms/button/button.svelte";
    import Icon from "../../atoms/icon/icon.svelte";
    let { onOpenSettings } = $props();
</script>

<section
    class="flex flex-col justify-between px-4 bg-black-100 max-w-fit h-screen"
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
