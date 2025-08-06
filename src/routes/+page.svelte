<script lang="ts">
  import SpaceAdmin from "@components/molecules/SpaceAdmin.svelte";
  import ToastContainer from "@components/molecules/ToastContainer.svelte";
  import Editor from "@components/organisms/Editor.svelte";
  import { Sidebar, Appbar, Workspace } from "@organisms";
  import { openAdminSpaces } from "@stores/workspace-store";
  import { onMount } from "svelte";

  let isTauri = false;
  onMount(() => {
    if (window && window.__TAURI_EVENT_PLUGIN_INTERNALS__) {
      isTauri = true;
    }
  });
</script>

<main class="min-h-screen flex flex-col bg-black-100 text-white" >
  {#if isTauri}
    <Appbar />
  {/if}
  {#if $openAdminSpaces}
    <SpaceAdmin />
  {/if}
  <section class="flex flex-1 min-h-0">
    <Sidebar />
    <Workspace />
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden mx-xs bg-black">
      <Editor />
    </div>
  </section>
  <ToastContainer  />
</main>
