<script lang="ts">
    import { ArrowLeftIcon, ExternalLink } from "@lucide/svelte";
    import { getVersion } from "@tauri-apps/api/app";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount, getContext } from "svelte";

    // Store the version of the app for displaying (fall back to Unknown if version can't be detected)
    let appVersion = $state("Unknown");

    // Fetch current app theme
    let theme = getContext<{ value: string }>("theme");

    // Runs when the page is opened
    onMount(async () => {
      // Fetch the version
      appVersion = await getVersion();
    });
</script>

<a
    href="/"
    class="back-btn"
    aria-label="Back"
    title="Back"
>
    <ArrowLeftIcon />
</a>

<h2>General</h2>
<div class="settings-card">
    <p class="setting-title" id="setting-title-theme">Theme</p>
    <select
        id="theme-dropdown"
        bind:value={theme.value}
    >
        <option value="light">Light</option>
        <option value="dark">Dark</option>
    </select>
</div>

<h2>About</h2>
<div class="settings-card">
    <p class="setting-title">Version</p>
    <p>{appVersion}</p>
</div>

<div class="settings-card">
    <p class="setting-title">License Information</p>
    <p>
        Licensed under the GNU General Public License Version 3
    </p>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <p class="link" onclick={() => openUrl('https://github.com/iamnotmega/writit/blob/main/LICENSE')}>
        Full license
        <ExternalLink size={14} strokeWidth={2} />
    </p>
</div>

<style>
    :global(p) {
        color: #999;
        padding-left: 8px;
        margin: 0;
    }

    :global(h2) {
        padding-left: 8px;
    }

    #setting-title-theme {
        margin-top: 4px;
    }

    #theme-dropdown {
        position: absolute;
        right: 20px;
        bottom: 9px;
        color-scheme: dark;
    }

    :global(body.light) #theme-dropdown {
        color-scheme: light;
    }

    .link {
        color: #f4f4f5;
        padding-top: 8px;
        cursor: pointer;
        display: inline-flex;
        align-items: center;
        gap: 4px;
        margin-top: 4px;
    }

    :global(body.light) .link {
        color: #222;
    }

    .link:hover {
        text-decoration: underline;
    }

    .setting-title {
        color: white;
        padding-bottom: 6px;
        padding-top: 2px;
    }

    :global(body.light) .setting-title {
        color: black;
    }

    .settings-card {
        background-color: #353535;
        border-radius: 4px;
        margin-left: 8px;
        margin-right: 8px;
        margin-bottom: 8px;
        height: fit-content;
        padding: 8px;
        position: relative;
    }

    :global(body.light) .settings-card {
        background-color: #eeeeee;
    }

    .back-btn {
        color: #666;
        transition: color 0.15s ease, color 0.15s ease;
        padding-top: 88px;
    }

    .back-btn:hover {
        color: white;
    }

    :global(body.light) .back-btn:hover {
        color: black;
    }
</style>
