<script lang="ts">
  import Logo from "@components/atoms/Logo.svelte";
  import AppbarWindowButtons from "@components/molecules/AppbarWindowButtons.svelte";
</script>
<nav class="w-full flex bg-black-100 z-30 justify-between relative" data-tauri-drag-region>
  <div class="pl-[12px] pointer-events-none">
    <Logo />
  </div>
  <AppbarWindowButtons />
</nav>