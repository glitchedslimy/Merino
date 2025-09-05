<script lang="ts">
    let { inputElement = $bindable(), inputType, class: className, value, onKeydown, onblur} = $props()
</script>

<input
    bind:this={inputElement}
    type={inputType}
    class="w-full h-full p-1 bg-black-300 text-brand-primary-light rounded-sm"
    value={value}
    onkeydown={onKeydown}
    onblur={onblur}
/>