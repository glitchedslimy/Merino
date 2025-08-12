<script lang="ts">
    import AiChat from "@components/molecules/AIChat.svelte";
    import "../assets/styles/tw.css";
    import SpaceAdmin from "@components/molecules/SpaceAdmin.svelte";
    import SyncBar from "@components/molecules/SyncBar.svelte";
    import ToastContainer from "@components/molecules/ToastContainer.svelte";
    import Editor from "@components/organisms/Editor.svelte";
    import { Sidebar, Appbar, Workspace } from "@organisms";
    import { openAdminSpaces } from "@stores/workspace-store";
    import { onMount, onDestroy } from "svelte";
    import { Ollama } from 'ollama/browser';
    import { startOllamaBridge } from "../lib/utils/ollama-bridge";
    import { get } from 'svelte/store';
    import { openAiChat } from "@stores/ai-store";
    
    let isTauri = false;
    let ollamaClient: Ollama | null = null;
    let isOllamaReady = false;

    // State variables for resizing
    let sidebarWidth = 250;
    let aiWidth = 250;
    let isResizingSidebar = false; // New separate flag
    let isResizingAiChat = false; // New separate flag

    // Handlers for sidebar resizing
    function handleSidebarMouseDown() {
        isResizingSidebar = true;
        document.body.style.cursor = 'ew-resize';
        document.body.style.userSelect = 'none';
        document.addEventListener('mousemove', handleSidebarMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    }

    function handleSidebarMouseMove(event: MouseEvent) {
        if (!isResizingSidebar) return;
        
        const newWidth = event.clientX;
        if (newWidth > 200 && newWidth < 500) {
            sidebarWidth = newWidth;
        }
    }

    // Handlers for AI Chat resizing
    function handleAiChatMouseDown() {
        isResizingAiChat = true;
        document.body.style.cursor = 'ew-resize';
        document.body.style.userSelect = 'none';
        document.addEventListener('mousemove', handleAiChatMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    }

    function handleAiChatMouseMove(event: MouseEvent) {
        if (!isResizingAiChat) return;

        const mainContainer = document.querySelector('section.flex') as HTMLElement;
        if (!mainContainer) return;
        
        const containerRight = mainContainer.getBoundingClientRect().right;
        const newWidth = containerRight - event.clientX;
        
        if (newWidth > 200 && newWidth < 500) {
            aiWidth = newWidth;
        }
    }

    // Common handler to stop resizing for both
    function handleMouseUp() {
        isResizingSidebar = false;
        isResizingAiChat = false;
        document.removeEventListener('mousemove', handleSidebarMouseMove);
        document.removeEventListener('mousemove', handleAiChatMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
        document.body.style.cursor = 'default';
        document.body.style.userSelect = 'auto';
        window.dispatchEvent(new Event('resize'));
    }

    onMount(() => {
        if (window && window.__TAURI_EVENT_PLUGIN_INTERNALS__) {
            isTauri = true;
        }
        startOllamaBridge().then(client => {
            console.log(`Ollama bridge started successfully...`);
            ollamaClient = client;
            isOllamaReady = true;
        }).catch(err => {
            console.error(`Failed to start Ollama bridge: ${err}`);
            isOllamaReady = false;
        });
    });

    onDestroy(() => {
        handleMouseUp();
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
                <Workspace style="width: {sidebarWidth}px;"/>
                 <div class="resizer horizontal" on:mousedown={handleSidebarMouseDown}></div>
                <div class="flex-1 flex flex-col overflow-hidden editor-wrapper">
                    <Editor />
                </div>
            </div>
            <SyncBar />
        </div>

      
            <div class={`resizer horizontal ${$openAiChat ? '' : 'hidden'}`} on:mousedown={handleAiChatMouseDown}></div>
                <AiChat style="width: {aiWidth}px;"/>
    </section>

    <ToastContainer />
</main>

<style>
    .sidebar-wrapper {
        flex-shrink: 0;
        overflow-y: auto;
        min-width: 200px;
        max-width: 500px;
    }

    .editor-wrapper {
        flex: 1;
        overflow: hidden;
    }

    .ai-chat-wrapper {
        flex-shrink: 0;
        min-width: 200px;
        max-width: 500px;
        overflow-y: auto;
    }

    .resizer {
        background: var(--color-black-100);
        position: relative;
        z-index: 10;
        transition: background 0.2s ease-in-out;
    }

    .resizer.horizontal {
        width: 6px;
        cursor: ew-resize;
    }

    .resizer:hover {
        background: none;
    }
</style>