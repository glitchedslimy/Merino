<script lang="ts">
    import Button from "../../atoms/button/button.svelte";
    import Icon from "../../atoms/icon/icon.svelte";
    import ThemeSwitch from "../../ThemeSwitch/ThemeSwitch.svelte";

    let { showSettingsModal = $bindable(), handleClose } = $props();
</script>

{#if showSettingsModal}
    <main class="absolute bg-black/90 backdrop-blur-xs h-screen top-8 w-full z-20">
        <div class="bg-black-100 px-10 py-5 rounded-md mx-10 my-10">
            <div class="flex justify-between items-center">
                <h1 class="font-bold text-2xl mb-2">Configuración</h1>
                <Button type="button" icon handleClick={() => handleClose()}>
                    <Icon iconName="close" />
                </Button>
            </div>
            <div class="flex flex-col">
                <h2 class="font-medium text-xl">Tema</h2>
                <ThemeSwitch />
            </div>
        </div>
    </main>
{/if}
