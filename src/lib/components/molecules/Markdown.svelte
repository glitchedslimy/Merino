<script lang="ts">
  import { marked } from "marked";
  export let text: string;
  let htmlContent: string = "";

  $: if (text) {
    htmlContent = marked.parse(text);
  }
</script>

<div>
  {@html htmlContent}
</div>

<style>

</style>