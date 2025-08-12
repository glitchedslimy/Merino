<script lang="ts">
    import { Logo } from "@atoms";
    import NotesTab from "@components/molecules/NotesTab.svelte";
    import {AppbarWindowButtons } from "@molecules"; 
</script>
<nav class="w-full flex  bg-black-100 z-30" data-tauri-drag-region>
    <div class="pl-[12px]">
        <Logo />
    </div>
   <AppbarWindowButtons />
</nav>