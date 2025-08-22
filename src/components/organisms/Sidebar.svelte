<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import NavigationBar from "@components/molecules/NavigationBar.svelte";

    // TODO Functionalities
</script>

<div class="h-full flex justify-between flex-col items-center px-md">
    <NavigationBar intent="vertical" spacing="v-md" justify="center" class="pt-md">
        <Button intent="icon">
            <Icon iconName="terminal" width="20" />
        </Button>
        <Button intent="icon">
            <Icon iconName="search" width="20" />
        </Button>
    </NavigationBar>
    <Button intent="icon" class="pb-md">
        <Icon iconName="settings" width="20" />
    </Button>
</div>
