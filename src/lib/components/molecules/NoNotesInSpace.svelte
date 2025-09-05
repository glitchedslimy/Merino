<script>
    import Button from "@components/atoms/Button.svelte";
<<<<<<< Updated upstream:src/lib/components/molecules/NoNotesInSpace.svelte
=======
    import { createNote } from "../../lib/actions/workspace/notes";
    import { activeSpace } from "../../lib/stores/workspace/spaces-store";
    import woolyLogo from "../../assets/icons/WoolyStrokeClear.svg"
>>>>>>> Stashed changes:src/components/organisms/NoNotesInSpace.svelte
</script>

<div
    class="bg-black h-screen rounded-md mt-xs flex justify-center items-center flex-col space-y-md"
>
    <div class="-mt-24 flex flex-col items-center justify-center space-y-md">
        <img
<<<<<<< Updated upstream:src/lib/components/molecules/NoNotesInSpace.svelte
            src="./src/assets/icons/WoolyStroke.svg"
=======
            src={woolyLogo}
>>>>>>> Stashed changes:src/components/organisms/NoNotesInSpace.svelte
            alt="Wooly Stroke Logo"
            width="250"
        />
        <h1 class="text-black-400 text-lg font-semibold">
            There aren't notes in the space or they are not open.
        </h1>
        <div class="flex flex-col space-y-sm items-center">
            <Button intent="link">Create a new note (Ctrl + N)</Button>
            <Button intent="link">Search for notes or spaces (Ctrl + K)</Button>
        </div>
    </div>
</div>
