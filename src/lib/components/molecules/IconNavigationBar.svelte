<script lang="ts">
    import { cva } from "class-variance-authority";
    const iconNavigation = cva('iconNavigation', {
        variants: {
            intent: {
                vertical: 'flex flex-col items-center justify-between items-center',
                horizontal: 'flex flex-row items-center items-center'
            },
            verticalSpacing: {
                xs: "space-y-xs",
                sm: 'space-y-sm',
                md: 'space-y-md',
                lg: 'space-y-lg',
                "xl": 'space-y-xl',
                "2xl": 'space-y-2xl',
                "3xl": 'space-y-3xl'   
            },
            horizontalSpacing: {
                xs: "space-x-xs",
                sm: 'space-x-sm',
                md: 'space-x-md',
                lg: 'space-x-lg',
                "xl": 'space-x-xl',
                "2xl": 'space-x-2xl',
                "3xl": 'space-x-3xl'  
            }
        }
    })
    let {verticalSpacing = null, horizontalSpacing = null, intent = null, children, class: className = ''  } = $props();
</script>
<div class={`${iconNavigation({ intent, verticalSpacing, horizontalSpacing })} ${className}`}>
    {@render children()}
</div>