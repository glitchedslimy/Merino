<script lang="ts">
  import { marked } from "marked";
  let {text} = $props();
  let htmlContent: string | Promise<string> = $state("");
    $effect(async () => {
      if (text) {
        htmlContent = await marked.parse(text);
      }
    })
    console.log(text)
</script>


{@html htmlContent}