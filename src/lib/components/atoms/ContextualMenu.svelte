<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  // Props are now defined reactively
  let { x, y, menuItems, contextId = '' } = $props();

  let menuElement: HTMLDivElement | undefined = undefined;
  const dispatch = createEventDispatcher();
  
  // Use $effect to handle the menu position and update reactively
  // This block runs whenever x, y, or menuElement changes
  $effect(() => {
    if (menuElement) {
      const menuRect = menuElement.getBoundingClientRect();
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      let newX = x;
      let newY = y;

      if (menuRect.right > viewportWidth) {
        newX = viewportWidth - menuRect.width - 10;
      }
      if (menuRect.bottom > viewportHeight) {
        newY = viewportHeight - menuRect.height - 10;
      }

      if (newX < 0) newX = 10;
      if (newY < 0) newY = 10;

      menuElement.style.left = `${newX}px`;
      menuElement.style.top = `${newY}px`;
    }
  });

  // Use $effect.root for managing side effects like event listeners
  $effect.root(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuElement && !menuElement.contains(event.target as Node)) {
        dispatch('close');
      }
    }

    document.addEventListener('click', handleClickOutside);

    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });

  function handleMenuItemClick(action: (id: string) => void) {
    action(contextId);
    dispatch('close');
  }
</script>

<style>
  .context-menu {
    position: absolute;
    background-color: white;
    border: 1px solid #e2e8f0;
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
    z-index: 50;
    min-width: 10rem;
  }

  .context-menu button {
    width: 100%;
    text-align: left;
    padding: 0.5rem 1rem;
    color: #4a5568;
    transition-property: background-color, border-color, color, fill, stroke, opacity, box-shadow, transform;
    transition-duration: 150ms;
    border-radius: 0.375rem;
    margin-left: 0.25rem;
    margin-right: 0.25rem;
    margin-top: 0.125rem;
    margin-bottom: 0.125rem;
    outline: none;
  }

  .context-menu button:hover,
  .context-menu button:focus {
    background-color: #f7fafc;
  }
</style>

<div
  bind:this={menuElement}
  class="absolute bg-black-100 flex flex-col px-xs py-xs rounded-md"
  style="left: {x}px; top: {y}px;"
  onclick={() => {}}
>
  {#each menuItems as item}
    <button onclick={() => handleMenuItemClick(item.action)} class="px-2 py-2 text-left rounded-sm hover:bg-black-200">
      {item.label}
    </button>
  {/each}
</div>