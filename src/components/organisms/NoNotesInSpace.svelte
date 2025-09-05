<script>
    import Button from "@components/atoms/Button.svelte";
    import { createNote } from "../../lib/actions/workspace/notes";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import woolyLogo from "../../assets/icons/WoolyStrokeClear.svg"
    import { showCommandPalette } from "../../lib/stores/commandpalette/commandpalette";
</script>

<div
    class="bg-black h-screen rounded-md mt-xs flex justify-center items-center flex-col space-y-md"
>
    <div class="-mt-24 flex flex-col items-center justify-center space-y-md">
        <img src={woolyLogo}
            alt="Wooly Stroke Logo"
            width="250"
        />
        <p class="text-black-400 text-2xl font-semibold">
            There aren't notes in the space or they are not open.
        </p>
        <div class="flex flex-col space-y-sm items-center">
            <Button intent="link" onClick={() => createNote($activeSpace ?? '', null)}>Create a new note (Ctrl + N)</Button>
            <Button intent="link" onClick={() => $showCommandPalette = !$showCommandPalette}>Search for notes or spaces (Ctrl + K)</Button>
        </div>
    </div>
</div>
