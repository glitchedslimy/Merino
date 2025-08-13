<script lang="ts">
  import { Logo } from "@atoms";
  import {AppbarWindowButtons } from "@molecules"; 
</script>
<nav class="w-full flex h-10 bg-black-100 z-30 justify-between" data-tauri-drag-region>
  <div class="pl-[12px] pointer-events-none">
    <Logo />
  </div>
  <AppbarWindowButtons />
</nav>