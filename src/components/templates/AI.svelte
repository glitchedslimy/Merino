<script lang="ts">
    import AiChatHeader from "@components/organisms/AIChatHeader.svelte";
    import { isOllamaRunning, openAiChat } from "../../lib/stores/ai/ai-store";
    import { slide } from "svelte/transition";
    import AIChatReponse from "@components/organisms/AIChatReponse.svelte";
    import AIChatInput from "@components/organisms/AIChatInput.svelte";
    import AiNotAvailable from "@components/molecules/AINotAvailable.svelte";
</script>

{#if $openAiChat}
    <section
        class="bg-black ml-xs flex flex-col"
        transition:slide={{ duration: 200, axis: "x" }}
    >
        {#if !$isOllamaRunning}
            <AiNotAvailable />
        {:else}
            <AiChatHeader />
            <AIChatReponse />
            <AIChatInput />
        {/if}
    </section>
{/if}
