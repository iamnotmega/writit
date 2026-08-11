<script lang="ts">
    import { invoke } from "@tauri-apps/api/core"

    let noteContent = $state("");
    let noteTitle = $state("");

    async function handleSave() {
        if (!noteTitle.trim()) return;

        await invoke("save_note", {
            name: noteTitle,
            contents: noteContent,
        });
    }
</script>

<aside class="sidebar">
    <h3 id="sidebar-heading">Notes</h3>
</aside>

<main class="editor-container">
    <div class="toolbar">
        <input
            type="text"
            bind:value={noteTitle}
            placeholder="Note title..."
            class="title-input"
        />
        <button onclick={handleSave} class="save-btn">Save</button>
    </div>

    <textarea
        bind:value={noteContent}
        class="note-input"
        placeholder="Write a note..."
    ></textarea>
</main>

<style>
    :global(html, body) {
        background-color: #1e1e1e;
        color: white;
        overflow: hidden;
        margin: 0;
    }

    .sidebar {
        width: 260px;
        height: 100vh;
        background-color: #353535;
        border-radius: 5px;
        position: fixed;
        top: 0;
        left: 0;
    }

    .editor-container {
        margin-left: 260px;
        width: calc(100% - 260px);
        height: 100vh;
        display: flex;
        flex-direction: column;
    }

    .toolbar {
        display: flex;
        gap: 8px;
        padding: 12px 16px;
        background-color: #252525;
    }

    .title-input {
        flex: 1;
        background: #1e1e1e;
        border: 1px solid #444;
        color: white;
        padding: 6px 10px;
        border-radius: 4px;
        outline: none;
    }

    .save-btn {
        background: #4a4a4a;
        color: white;
        border: none;
        padding: 6px 14px;
        border-radius: 4px;
        cursor: pointer
    }

    .save-btn:hover {
        background: #5a5a5a;
    }

    .note-input {
        flex: 1;
        background-color: #1e1e1e;
        color: white;
        outline: none;
        border: none;
        resize: none;
        padding: 16px;
        font-family: inherit;
        font-size: 1rem;
    }

    #sidebar-heading {
        padding-left: 8px; 
    }
</style>