<script lang="ts">
    import { setContext, onMount } from "svelte";

    let { children } = $props();
    let theme = $state("dark"); // Current app theme

    // Share current theme value with all pages
    setContext("theme", {
      get value() { return theme; }, // Return current theme
      set value(newTheme) { theme = newTheme; localStorage.setItem("theme", newTheme); } // Save current theme
    });

    // Load saved theme when the app starts
    onMount(() => {
      // Fetch saved theme
      const savedTheme = localStorage.getItem("theme");

      // Set the current theme to the saved theme
      if (savedTheme === "light" || savedTheme === "dark") {
        theme = savedTheme;
      }
    });

    // Handle theme changes
    $effect(() => { // Run whenever theme is changed
      document.body.className = theme;
    });
</script>

{@render children()}

<style>
    /* Dark theme */
    :global(body) {
        background-color: #1e1e1e;
        color: white;
        margin: 0;
    }

    /* Light theme */
    :global(body.light) {
        background-color: white;
        color: black;
    }
</style>
