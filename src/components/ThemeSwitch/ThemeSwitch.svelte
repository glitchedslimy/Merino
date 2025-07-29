<script lang="ts">
    let selectedTheme = $state('default');
    $effect(() => {
        if(typeof document !== 'undefined') {
            if(selectedTheme === 'default') {
                document.documentElement.removeAttribute('data-theme');
            } else {
                document.documentElement.setAttribute('data-theme', selectedTheme)
            }
        }
    })
</script>
<div>
   <select id="themes" name="themes" bind:value={selectedTheme}>
        <option value="default">Default</option>
        <option value="Catppucin">Catppucin</option>
   </select>
</div>
