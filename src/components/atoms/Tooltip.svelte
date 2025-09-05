<script lang="ts">
  let {text, x, y} = $props()
</script>

<div class="fixed p-sm bg-black-100 text-white rounded-md z-10 text-sm pointer-events-none whitespace-nowrap" style="left: {x}px; top: {y}px;">
  {text}
</div>