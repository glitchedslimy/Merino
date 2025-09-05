<script lang="ts">
    import { cva } from "class-variance-authority";
    /**
     * Create the CVA TS for the button to be able to change its styles
     * just using intents.
    */
   const button = cva('button', {
    variants: {
        intent: {
            icon: "text-white hover:text-white-300 hover:cursor-pointer",
            iconStop: "text-brand-primary-light hover:cursor-pointer",
            primary: "text-white hover:bg-brand-primary/60 hover:cursor-pointer bg-brand-primary px-5 py-2 rounded-md font-medium transition duration-200 w-full",
            secondary: "text-white hover:bg-brand-secondary-dark px-5 py-2 rounded bg-brand-secondary cursor-pointer font-medium transition duration-200 w-2xl",
            success: "text-white hover:bg-utils-green-dark px-5 py-2 rounded bg-utils-green cursor-pointer font-medium transition duration-200 w-2xl",
            warning: "text-black hover:bg-utils-yellow-dark px-5 py-2 rounded bg-utils-yellow cursor-pointer font-medium transition duration-200 w-2xl",
            danger: "text-white hover:bg-utils-red-dark px-5 py-2 rounded bg-utils-red cursor-pointer font-medium transition duration-200 w-2xl",
            gray: "text-white hover:bg-black-300 px-5 py-2 rounded bg-black-200 cursor-pointer font-medium transition duration-200 w-full",
            window: 'hover:bg-black-200 transition duration-200 px-4 py-1',
            windowClose: 'hover:bg-utils-red transition duration-200 px-4 py-1',
            text: 'text-black-400 cursor-pointer font-medium transition duration-200 w-2xl hover:text-white rounded-md',
            notes: 'flex space-x-xs items-center text-black-300 hover:bg-black-200 px-1 py-1 w-full rounded-md mt-1',
            link: 'text-brand-primary hover:text-brand-primary-light cursor-pointer duration-200 transition',
            dropdown: "w-full flex items-center justify-between space-x-md px-sm py-sm text-left  rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-brand-primary focus:border-brand-primary transition-all duration-300 ease-in-out hover:bg-black-200 group",
            dropdownText: 'text-md text-black-400 px-5 py-2 cursor-pointer transition duration-200 w-2xl hover:bg-black-200 hover:text-white rounded-md',
            settings: 'hover:bg-black-200 duration-200 transition-all py-sm px-md rounded-md hover:cursor-pointer w-full justify-start flex'
        }
    }
   })

   /**
    * Define the props for the component (Rune mode)
    * Defined the handleClick to properly pass functions to the button to do
    * certain stuff without the need to do multiple molecules.
   */
  let { intent = null, children, onClick= () => {}, class: className = '', disabled = false, onmouseenter = null, onmouseleave = null, draggable = false, ondragstart = null, ondrop = null, ondragover = null, ondragenter = null, ondragleave = null, oncontextmenu = null} = $props();
</script>

<button class={`${button({ intent })} ${className}`} onclick={() => onClick()} disabled={disabled} {onmouseenter} {onmouseleave} {draggable} {ondragstart} {ondrop} {ondragover} {ondragenter} {ondragleave} {oncontextmenu}>
    {@render children()}
</button>