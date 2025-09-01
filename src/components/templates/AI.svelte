<script>
    import AiChatHeader from "@components/organisms/AIChatHeader.svelte";
    import { openAiChat } from "../../lib/stores/ai/ai-store";
    import { slide } from "svelte/transition";
    import AIChatReponse from "@components/organisms/AIChatReponse.svelte";
    import AIChatInput from "@components/organisms/AIChatInput.svelte";

</script>
{#if $openAiChat}
<section class="bg-black ml-xs flex flex-col" transition:slide={{ duration: 200, axis: "x"}}>
    <AiChatHeader />
    <AIChatReponse />
    <AIChatInput />
</section>
{/if}