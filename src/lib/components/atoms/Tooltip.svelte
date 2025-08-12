<script lang="ts">
  let {text, x, y} = $props()
</script>

<div class="custom-tooltip" style="left: {x}px; top: {y}px;">
  {text}
</div>

<style>
  .custom-tooltip {
    position: fixed;
    padding: 0.5rem;
    background-color: var(--color-black-100);
    color: var(--color-white);
    border-radius: 4px;
    z-index: 1000;
    font-size: 0.875rem;
    pointer-events: none;
    white-space: nowrap;
  }
</style>