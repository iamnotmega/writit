<script lang="ts">
    let { children } = $props();
</script>

{@render children()}

<style>
    :global(html, body) {
        background-color: #1e1e1e;
        color: white;
        margin: 0;
    }
</style>