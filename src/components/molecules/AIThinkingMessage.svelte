<script lang="ts">
    import Icon from "@components/atoms/Icon.svelte";
    import { aiMessages } from "../../lib/stores/ai/ai-store";
    import Markdown from "@components/atoms/Markdown.svelte";

    let { message, index } = $props();
</script>

{#if message.sender === 'ai' && message.thinking}
      <div class="flex justify-start mb-4">
        <div class="rounded-lg px-4 py-2 text-black-300 text-sm self-start max-w-[212px]">
          <button
            onclick={() => {
              aiMessages.update(msgs => {
                msgs[index].isThinkingDropdownOpen = !msgs[index].isThinkingDropdownOpen;
                return msgs;
              });
            }}
            class="flex items-center gap-2 p-1"
          >
            <div class="font-bold">Show Thinking</div>
            <Icon iconName="chevron-down" width="16" height="16" />
          </button>
          
          {#if message.isThinkingDropdownOpen}
            <div class="mt-2 p-2 rounded-lg text-black-300 italic">
              <Markdown text={message.thinking} />
            </div>
          {/if}
        </div>
      </div>
    {/if}