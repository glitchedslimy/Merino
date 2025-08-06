<script lang="ts">
  import { afterUpdate, onMount, onDestroy } from 'svelte';
  import { createEventDispatcher } from 'svelte';

  // Props received from the parent component
  export let x; // X-coordinate for menu position
  export let y; // Y-coordinate for menu position
  export let menuItems = []; // Array of menu items { label: string, action: function }
  export let contextId: string | null = null; // Optional ID related to the context (e.g., note ID)

  let menuElement; // Reference to the menu's DOM element

  const dispatch = createEventDispatcher();

  // Function to dispatch a custom event when a menu item is clicked
  function handleMenuItemClick(action, id) {
    // Execute the action provided by the parent
    action(id);
    // Notify the parent to close the menu
    dispatch('close');
  }

  // Adjust menu position if it goes off-screen
  afterUpdate(() => {
    if (menuElement) {
      const menuRect = menuElement.getBoundingClientRect();
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      let newX = x;
      let newY = y;

      // Check if menu extends beyond right edge
      if (menuRect.right > viewportWidth) {
        newX = viewportWidth - menuRect.width - 10; // 10px padding from right edge
      }
      // Check if menu extends beyond bottom edge
      if (menuRect.bottom > viewportHeight) {
        newY = viewportHeight - menuRect.height - 10; // 10px padding from bottom edge
      }

      // Ensure menu doesn't go off left/top if initial position is too close to edge
      if (newX < 0) newX = 10;
      if (newY < 0) newY = 10;

      // Apply new position if it has changed
      if (menuElement.style.left !== `${newX}px` || menuElement.style.top !== `${newY}px`) {
        menuElement.style.left = `${newX}px`;
        menuElement.style.top = `${newY}px`;
      }
    }
  });

  // Handle clicks outside the menu to close it
  function handleClickOutside(event) {
    // If the click is outside the menu element, dispatch a close event
    if (menuElement && !menuElement.contains(event.target)) {
      dispatch('close');
    }
  }

  // Add event listener when component mounts
  onMount(() => {
    document.addEventListener('click', handleClickOutside);
  });

  // Remove event listener when component unmounts to prevent memory leaks
  onDestroy(() => {
    document.removeEventListener('click', handleClickOutside);
  });
</script>

<style>
  /* Basic styling for the menu container */
  .context-menu {
    position: absolute;
    background-color: white;
    border: 1px solid #e2e8f0; /* gray-200 */
    border-radius: 0.5rem; /* rounded-lg */
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05); /* shadow-xl */
    padding-top: 0.5rem; /* py-2 */
    padding-bottom: 0.5rem; /* py-2 */
    z-index: 50; /* z-50 */
    min-width: 10rem; /* min-w-[160px] */
  }

  /* Styling for individual menu items (buttons) */
  .context-menu button {
    width: 100%;
    text-align: left;
    padding: 0.5rem 1rem; /* px-4 py-2 */
    color: #4a5568; /* gray-700 */
    transition-property: background-color, border-color, color, fill, stroke, opacity, box-shadow, transform;
    transition-duration: 150ms; /* transition-colors duration-150 */
    border-radius: 0.375rem; /* rounded-md */
    margin-left: 0.25rem; /* mx-1 */
    margin-right: 0.25rem; /* mx-1 */
    margin-top: 0.125rem; /* my-0.5 */
    margin-bottom: 0.125rem; /* my-0.5 */
    outline: none; /* focus:outline-none */
  }

  .context-menu button:hover,
  .context-menu button:focus {
    background-color: #f7fafc; /* gray-100 */
  }
</style>

<div
  bind:this={menuElement}
  class="context-menu"
  style="left: {x}px; top: {y}px;"
  onclick={() => {}}
>
  {#each menuItems as item, index}
    <button
      onclick={() => handleMenuItemClick(item.action, contextId)}
    >
      {item.label}
    </button>
  {/each}
</div>
