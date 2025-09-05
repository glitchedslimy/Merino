<script>
    import EditorFileName from "@components/molecules/EditorFileName.svelte";
    import EditorForwardBackward from "@components/molecules/EditorForwardBackward.svelte";
    import EditorQuickActions from "@components/molecules/EditorQuickActions.svelte";

</script>
<div class="flex justify-between items-center px-sm py-sm mb-xs bg-black rounded-md">
   <EditorForwardBackward />
   <EditorFileName />
   <EditorQuickActions />
</div>