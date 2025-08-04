<script lang="ts">
  import {Commandbar, TitleBar, Settingsmodal, Sidebar, Utilsbar} from "@organisms";
  import Editor from "@components/editor/editor.svelte";
  import EditorJS from "@editorjs/editorjs";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { initEditor } from "../shared/scripts/editor/initEditor";
  // State variables and functions remain the same
  
  
  let editorjs: EditorJS | null = null;
  const editor = initEditor(editorjs);

  // Logic for the space selector
  let currentSpace = { name: 'Default' };
  let availableSpaces = [];
  
  async function fetchSpaces() {
    let promiseSpaces = invoke('list_spaces_cmd');
    availableSpaces = await promiseSpaces
    const savedSpaceName = localStorage.getItem('currenSpaceName');
    if(savedSpaceName) {
      const resolvedPromise = await promiseSpaces
      const foundSpace = resolvedPromise.find(s => s.name === savedSpaceName);
      if(foundSpace) {
        currentSpace = foundSpace;
      }
    } else if((await promiseSpaces).lenght > 0) {
      currentSpace = (await promiseSpaces)[0]
    }
  }
  
  function handleSpaceSelected(selectedSpace) {
    currentSpace = selectedSpace;
    localStorage.setItem('currentSpaceName', selectedSpace.name);
    
  }

  onMount(() => {
    fetchSpaces()
    invoke('create_space_cmd', { req: { name: "spaces" }})
    const holderElement = document.getElementById("editorjs");
    if (!holderElement) {
      console.error("Editor.js holder element not found!");
      return; // Exit if holder is not found
    }
  });
</script>

<main class="min-h-screen flex flex-col bg-black-100 text-white">
  <TitleBar />
  <Settingsmodal />
  <section class="flex flex-1 min-h-0">
    <Commandbar />
    <Sidebar
      currentSpaceName={currentSpace.name}
      availableSpaces={availableSpaces}
      onSpaceSelected={handleSpaceSelected}
      className="flex-shrink-0 w-64 md:w-72 lg:w-80"
    />
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden mx-1">
      <Utilsbar {editor}/>
      <Editor />
    </div>
  </section>
</main>
