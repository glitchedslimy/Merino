<script lang="ts">
  import Markdown from "@components/atoms/Markdown.svelte";
  import { aiIsLoading, aiMessages, chatContainer, selectedModel } from "../../lib/stores/ai/ai-store";
    import AiThinkingMessage from "@components/molecules/AIThinkingMessage.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { t } from "$lib/i18n";
</script>

<div class="flex-1 overflow-y-auto p-4 rounded-lg shadow-inner" bind:this={$chatContainer}>
  {#each $aiMessages as message, index}
    <div class="flex flex-col {message.sender === 'user' ? 'justify-end' : 'justify-start'} mb-4">
      <AiThinkingMessage message={message} index={index} />
      <div
        class="rounded-lg px-4 py-2 {message.sender === 'user' ? 'bg-brand-primary text-white-100 self-end' : 'bg-black-100 text-white-100 self-start'} max-w-[212px]"
      >
        <div class="font-bold">
          {#if message.sender === 'user'}
          You
          {:else}
            <div class="flex gap-x-sm items-center justify-start">
              <Icon iconName="WoolyStroke" width="16"/>
              <p>{$t('ai.ai')}</p>
            </div>
          {/if}
        </div>
        <Markdown text={message.text} />
      </div>
    </div>
  {/each}

  {#if $aiIsLoading}
    <div class="flex justify-center items-center gap-x-xs animate-pulse">
      <p class="text-black-300 text-sm">{$t('ai.aiThinking')}</p>
    </div>
  {/if}
</div>