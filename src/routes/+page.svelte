<script lang="ts">
  import AiChat from "@components/molecules/AIChat.svelte";
  import "../assets/styles/tw.css";
  import SpaceAdmin from "@components/molecules/SpaceAdmin.svelte";
  import SyncBar from "@components/molecules/SyncBar.svelte";
  import ToastContainer from "@components/molecules/ToastContainer.svelte";
  import Editor from "@components/organisms/Editor.svelte";
  import { Sidebar, Appbar, Workspace } from "@organisms";
  import { openAdminSpaces } from "@stores/workspace-store";
  import { onMount } from "svelte";
  import { Ollama } from 'ollama';
    import { startOllamaBridge } from "../lib/utils/ollama-bridge";
    import Button from "@components/atoms/Button.svelte";

  let isTauri = false;
   let ollamaClient: Ollama | null = null;
  let isOllamaReady = false;

  onMount(() => {
    if (window && window.__TAURI_EVENT_PLUGIN_INTERNALS__) {
      isTauri = true;
    }
     startOllamaBridge().then(client => {
      console.log(`Ollama bridge started successfully...`);
      ollamaClient = client; // Store the client instance
      isOllamaReady = true;  // Set the flag to true
    }).catch(err => {
      console.error(`Failed to start Ollama bridge: ${err}`);
      isOllamaReady = false;
    });
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
        <AiChat />
      </div>
      <SyncBar />
    </div>
  </section>

  <ToastContainer />
</main>
