<script>
    import Button from "@components/atoms/Button.svelte";
    import NavigationBar from "./NavigationBar.svelte";
    import Icon from "@components/atoms/Icon.svelte";

</script>
<NavigationBar intent="horizontal" spacing="h-sm">
    <Button intent="icon">
        <Icon iconName="read" width="20" />
    </Button>
    <Button intent="icon">
        <Icon iconName="dotsvertical" width="20" />
    </Button>
</NavigationBar>