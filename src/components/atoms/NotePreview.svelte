<script lang="ts">
    import Markdown from "./Markdown.svelte";

    let { text, x, y, onmouseenter, onmouseleave } = $props();
</script>

<div
    {onmouseenter}
    {onmouseleave}
    class="fixed p-sm bg-black-100 text-white rounded-md z-10 text-sm whitespace-pre-wrap w-[30rem] shadow-lg transition-opacity duration-200 max-h-[20rem] overflow-y-auto"
    style="left: {x}px; top: {y}px;"
>
    <Markdown {text} />
</div>
