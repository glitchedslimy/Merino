<script lang="ts">
    import { Logo } from "@atoms";
    import {AppbarWindowButtons } from "@molecules"; 
</script>
<nav class="w-full flex items-center justify-between bg-black-100 z-20" data-tauri-drag-region>
    <div class="pl-[12px]">
        <Logo />
    </div>
   <AppbarWindowButtons />
</nav>