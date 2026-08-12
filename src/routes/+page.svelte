<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let noteContent = $state("");
    let noteTitle = $state("");
    let activeNote = $state(""); // The note currently selected

    let saveText = $state("Save"); // Current text displayed on the save button
    let isSaving = $state(false); // State of the saving operation

    let notes = $state<string[]>([]); // List of saved notes

    // Attempt to load the list of notes
    async function loadNotes() {
        try {
            notes = await invoke("get_notes"); // Invoke backend command    
        } catch (err) { // Print to console on error
            console.error("Failed to fetch notes:", err);
        }
    }

    // Handle note selection
    async function selectNote(name: string) {
        try {
        // Fetch note contents using backend
        const contents = await invoke<string>("read_note", { name });

        // Replace editor contents with loaded note
        noteTitle = name;
        noteContent = contents;

        // Set the note as the current selected note
        activeNote = name;
        } catch (err) { // Print to console on error
            console.error("Failed to read note:", err);
        }
    }

    // Handle saving notes
    async function handleSave() {
        if (!noteTitle.trim() || isSaving) return;

        isSaving = true;
        saveText = "Saving..."

        try { // Attempt to save the note to the disk
            await invoke("save_note", { // Invoke backend command, passing the note's title and contents
                name: noteTitle,
                contents: noteContent,
            });

            saveText = "Success!"
            await loadNotes(); // Refresh the list of notes after saving a new note
        } catch { // Run on error
            saveText = "Error!"; 
        } finally {
            setTimeout(() => { // Reset status text back to normal after 1.5s 
                saveText = "Save";
                isSaving = false;
            }, 1500);
        }
    }

    // Load the list of notes on startup
    onMount(() => {
        loadNotes();
    });
</script>

<aside class="sidebar">
    <h3 id="sidebar-heading">Notes</h3>
    <div class="notes-list">
    {#each notes as note}
      <button 
        class="note-btn {activeNote === note? "active" : ''}"
        onclick={() => selectNote(note)}
      >
        {note}
      </button>
    {:else}
      <p class="empty">No notes yet...</p>
    {/each}
  </div>
</aside>

<main class="editor-container">
    <div class="toolbar">
        <input
            type="text"
            bind:value={noteTitle}
            placeholder="Note title..."
            class="title-input"
        />
        <button onclick={handleSave} class="save-btn">{saveText}</button>
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

    .notes-list {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 6px;
        padding-top: 8px;
    }

    .note-btn {
        width: 240px;
        padding: 8px 12px;
        background: #252525;
        color: #e0e0e0;
        border: 1px solid #444;
        border-radius: 4px;
        font-size: 0.9rem;
        text-align: left;
        cursor: pointer;
        transition: background 0.15s ease, border-color 0.15s ease;
    }

    .note-btn:hover,
    .note-btn:active {
        background: #4a4a4a;
        color: white;
    }

    .empty {
        color: #888;
        font-size: 0.85rem;
        padding-left: 12px;
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