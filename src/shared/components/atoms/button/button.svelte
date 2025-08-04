<script lang="ts">

 let { icon = false, type, handleClick = () => {}, children = ()=>{}, transition = () => {}, dropdown = false  } = $props();
 let classes = $state('')
 
    if(icon) {
        classes = "text-white hover:text-white-300 hover:cursor-pointer"
    } 
    if(!icon) {
        classes = "text-black hover:bg-brand-primary/60 hover:cursor-pointer bg-brand-primary px-5 py-2 rounded-md font-medium"
    } 
    if(dropdown) {
         classes = "text-white hover:text-white-300 hover:cursor-pointer flex space-x-2 items-center"
    }
</script>


<button {type} class={classes} onclick={() => handleClick()} >
    {@render children()}
</button>