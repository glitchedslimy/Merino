<script>
    import AiChat from '@components/organisms/AiChat.svelte';
    import '../assets/styles/tw.css'
    import Appbar from "@components/organisms/Appbar.svelte";
    import Editor from '@components/templates/Editor.svelte';
    import Sidebar from "@components/organisms/Sidebar.svelte";
    import Editorbar from '@components/organisms/Editorbar.svelte';
    import Workspace from '@components/templates/Workspace.svelte';
    import Ai from '@components/templates/AI.svelte';

</script>
<!-- <main class="grid grid-rows-[auto_1fr] h-screen overflow-hidden">
  <Appbar />
  <div class="grid grid-cols-[3.5rem_1fr] h-full">
    <div class="h-full">
      <Sidebar />
    </div>
    
    <div class="grid grid-rows-[1fr_auto] h-full">
      <div class="bg-black-100 text-white grid grid-cols-[18rem_70rem_auto] gap-xs">
        <div>
          <Workspace />
        </div>
        <div>
          <Editor />
        </div>
        <div>
          <AiChat />
        </div>
      </div>
      <Editorbar />
    </div>
  </div>
</main> -->

<main class="bg-black-100 text-white h-screen flex flex-col">
    <Appbar />
    <section class="flex flex-1 min-h-0">
            <Sidebar />
        <div class="flex-1 flex flex-col min-w-0">
            <!-- Admin Spaces here -->
            <div class="flex flex-1 min-h-0">
                <Workspace />
                 <!-- Resizer here -->
                <div class="flex-1 flex flex-col overflow-hidden editor-wrapper">
                    <Editor />
                </div>
            </div>
            <Editorbar />
        </div>

      
            <!-- <div class={`resizer horizontal ${$openAiChat ? '' : 'hidden'}`} on:mousedown={handleAiChatMouseDown}></div>
                <AiChat style="width: {aiWidth}px;"/> -->
        <Ai />
    </section>
</main>