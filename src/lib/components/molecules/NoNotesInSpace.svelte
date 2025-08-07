<script>
    import Button from "@components/atoms/Button.svelte";
</script>

<div
    class="bg-black h-screen rounded-md mt-xs flex justify-center items-center flex-col space-y-md"
>
    <div class="-mt-24 flex flex-col items-center justify-center space-y-md">
        <img
            src="./src/assets/WoolyStroke.svg"
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
