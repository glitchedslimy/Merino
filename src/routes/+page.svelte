<script lang="ts">
  import '../assets/styles/tw.css'
  import SpaceAdmin from "@components/molecules/SpaceAdmin.svelte";
    import SyncBar from "@components/molecules/SyncBar.svelte";
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

<main class="bg-black-100 text-white h-screen flex flex-col">
  {#if isTauri}
    <Appbar />
  {/if}

  <section class="flex flex-1 min-h-0">
    <Sidebar />
    <div class="flex-1 flex flex-col min-w-0">
      {#if $openAdminSpaces}
        <SpaceAdmin />
      {/if}
      <div class="flex flex-1 min-h-0">
        <Workspace />
        <div class="flex-1 flex flex-col overflow-hidden mx-xs">
          <Editor />
        </div>
      </div>
      <SyncBar />
    </div>
  </section>

  <ToastContainer />
</main>
