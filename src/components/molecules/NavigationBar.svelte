<script lang="ts">
    import { cva } from "class-variance-authority";

    const navbar = cva("merino__navbar", {
        variants: {
            intent: {
                vertical: "flex flex-col",
                horizontal: "flex flex-row"
            },
            spacing: {
                "h-xs": 'gap-x-sx',
                "h-sm": 'gap-x-sm',
                "h-md": 'gap-x-md',
                "h-lg": 'gap-x-lg',
                "h-xl": 'gap-x-xl',
                "h-2xl": 'gap-x-2xl',
                "h-3xl": 'gap-x-3xl',
                "v-xs": 'gap-x-sx',
                "v-sm": 'gap-x-sm',
                "v-md": 'gap-y-md',
                "v-lg": 'gap-y-lg',
                "v-xl": 'gap-y-xl',
                "v-2xl": 'gap-y-2xl',
                "v-3xl": 'gap-y-3xl'
            },
            justify: {
                center: "justify-center items-center"
            }
        }
    })

    let { intent = null, children, class: className = "", spacing = null, justify = null } = $props();
</script>
<div class={`${navbar({ intent, spacing, justify })} ${className}`}>
    {@render children()}
</div>