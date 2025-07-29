<script lang="ts">

 let { icon = false, type, handleClick = () => {}, children = ()=>{}} = $props();
 let classes = $state('')
 
    if(icon) {
        classes = "text-white hover:text-white-300 hover:cursor-pointer"
    } else {
        classes = "text-black hover:bg-brand-primary/60 hover:cursor-pointer bg-brand-primary px-5 py-2 rounded-md font-medium"
    }
</script>


<button {type} class={classes} onclick={() => handleClick()}>
    {@render children()}
</button>