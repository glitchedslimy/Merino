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
            primary: "text-white hover:bg-brand-primary/60 hover:cursor-pointer bg-brand-primary px-5 py-2 rounded-md font-medium transition duration-200 w-2xl",
            secondary: "text-white hover:bg-brand-secondary-dark px-5 py-2 rounded bg-brand-secondary cursor-pointer font-medium transition duration-200 w-2xl",
            success: "text-white hover:bg-utils-green-dark px-5 py-2 rounded bg-utils-green cursor-pointer font-medium transition duration-200 w-2xl",
            warning: "text-black hover:bg-utils-yellow-dark px-5 py-2 rounded bg-utils-yellow cursor-pointer font-medium transition duration-200 w-2xl",
            danger: "text-white hover:bg-utils-red-dark px-5 py-2 rounded bg-utils-red cursor-pointer font-medium transition duration-200 w-2xl",
            gray: "text-white hover:bg-black-300 px-5 py-2 rounded bg-black-200 cursor-pointer font-medium transition duration-200 w-2xl",
            appbar: 'hover:bg-black-200 transition duration-200 px-4 py-1',
            appbarclose: 'hover:bg-utils-red transition duration-200 px-4 py-1',
            ghost: 'text-sm text-black-400 px-5 py-2 cursor-pointer font-medium transition duration-200 w-2xl hover:bg-black-200 hover:text-white',
            notes: 'flex space-x-xs items-center text-black-300 hover:bg-black-200 px-1 py-1 w-full rounded-md mt-1',
            link: 'text-brand-primary hover:text-brand-primary-light cursor-pointer duration-200 transition'
        }
    }
   })

   /**
    * Define the props for the component (Rune mode)
    * Defined the handleClick to properly pass functions to the button to do
    * certain stuff without the need to do multiple molecules.
   */
  let { intent = null, children, handleClick= () => {}, class: className = '', disabled = false } = $props();
</script>

<button class={`${button({ intent })} ${className}`} onclick={() => handleClick()} disabled={disabled}>
    {@render children()}
</button>