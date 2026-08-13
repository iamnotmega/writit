<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import { Trash2Icon, SquarePenIcon } from "@lucide/svelte";

    let saveTimer: ReturnType<typeof setTimeout> | undefined;
    let noteContent = $state("");
    let noteTitle = $state("Untitled");
    let activeNote = $state(""); // The note currently selected
    let notes = $state<string[]>([]); // List of saved notes

    let historyTimer: ReturnType<typeof setTimeout> | undefined;    
    let undoStack = $state<string[]>([]);
    let redoStack = $state<string[]>([]);

    function saveHistory() {
        // Only save if the content actually changed
        if (noteContent !== undoStack[undoStack.length -1]) {
            undoStack.push(noteContent); // Push current note content to undo history so it can be reverted
            redoStack = []; // Clear redo history
        }
    }

    // Handle undo operation
    function undo() {
        clearTimeout(historyTimer); // Cancel pending history timer
        saveHistory(); // Save current state immediately

        if (undoStack.length <= 0) return; // Need atleast 2 states for undo

        redoStack.push(undoStack.pop()!); // Move current state to redo stack
        noteContent = undoStack[undoStack.length -1]!; // Load previous note state
    }

    // Handle redo operation
    function redo() {
        if (redoStack.length === 0) return; // Stop the function if there is nothing to redo

        const nextState = redoStack.pop()!; // Get most recent undone note content
        undoStack.push(nextState); // Move back to undo history so it can be undone again
        noteContent = nextState; // Replace current note content with most recent undo
    }

    // Function to handle keyboard input for keybinds
    function handleKeydown(event: KeyboardEvent) { // Function receives a keyboard event
        const modifier = event.ctrlKey || event.metaKey;

        if (modifier && event.key === "z" && !event.shiftKey) { // Run if Ctrl/Cmd + Z was pressed
            event.preventDefault(); // Prevent browser native function from interfering
            undo(); // Execute the undo operation
        }

        if (modifier && event.key === "z" && event.shiftKey) { // Run if Ctrl/Cmd + Shift + Z was pressed
            event.preventDefault();
            redo(); // Execute the redo operation
        }

        if (modifier && event.key === "y") { // Run if Ctrl/Cmd + Y was pressed
            event.preventDefault();
            redo();
        }
    }

    // Attempt to load the list of notes
    async function loadNotes() {
        try {
            notes = await invoke("get_notes"); // Invoke backend command    
        } catch (err) { // Print to console on error
            console.error("Failed to fetch notes:", err);
        }
    }

    // Function to automatically save notes
    function handleInput() {
        // Clear history timer if the user is still typing
        clearTimeout(historyTimer);

        // Wait after last keystroke before saving to history
        historyTimer = setTimeout(() => {
            saveHistory();
        }, 500);

        // Clear previous timer if the user is still actively typing
        clearTimeout(saveTimer);

        // Otherwise wait after last keystroke before saving the note
        saveTimer = setTimeout(() => {
            handleSave();
        }, 300);
    }

    // Handle note creation
    function createNewNote() {
        clearTimeout(saveTimer); // Clear save timer

        // Reset note content and title
        activeNote = "";
        noteTitle = "Untitled";
        noteContent = "";

        // Clear note history
        undoStack = [];
        redoStack = [];
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

        // Clear note history
        undoStack = [];
        redoStack = [];
        } catch (err) { // Print to console on error
            console.error("Failed to read note:", err);
        }
    }

    // Handle saving notes
    async function handleSave() {
        const title = noteTitle.trim() || "Untitled"

        try { // Attempt to save the note to the disk

            // If the current note's title changed, rename the file
            if (activeNote && activeNote !== title) {
                await invoke("delete_note", { name: activeNote });
            }

            await invoke("save_note", { // Invoke backend command, passing the note's title and contents
                name: title,
                contents: noteContent,
            });

            activeNote = title; // Set active note
            await loadNotes(); // Refresh the list of notes after saving a new note
        } catch (err) { // Run on error
            console.log("Failed to save note:", err);
        }
    }

    // Handle deleting notes
    async function handleDelete(name: string) {
        if (!name || !name.trim()) return;

        // Attempt to delete the note from disk
        try {
            // Invoke backend command to delete the note
            await invoke("delete_note", { name });

            // Remove file extension
            const clean = (str:string) => str.replace(/\.md$/, "");

            // Clear editor if current note was deleted
            if (clean(activeNote) === clean(name) || clean(noteTitle) === clean(name)) {
                noteTitle = "Untitled";
                noteContent = "";
                activeNote = "";
            }

            // Refresh note list
            await loadNotes();
        } catch (err) { // Print to console on error
            console.error("Failed to delete note:", err)
        }
    }

    // Load the list of notes on startup
    onMount(() => {
        loadNotes();
    });
</script>

<aside class="sidebar">
    <button
        class="new-btn"
        aria-label="New note"
        onclick={() => createNewNote()}
        >
        <SquarePenIcon />
        </button>
    <div class="notes-list">
    {#each notes as note}
      <div class="note-item">
        <button 
        class="note-btn {activeNote === note? "active" : ''}"
        onclick={() => selectNote(note)}
      >
        {note}
      </button>
      <button
        class="delete-btn"
        onclick={() => handleDelete(note)}
        aria-label="Delete note"
        >
        <Trash2Icon />
        </button>
      </div>
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
            oninput={handleInput}
            class="title-input"
        />
    </div>

    <textarea
        bind:value={noteContent}
        oninput={handleInput}
        onkeydown={handleKeydown}
        class="note-input"
    ></textarea>

    <div class="preview">
        <SvelteMarkdown source={noteContent} options={{ breaks: true }} />
    </div>
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

    .new-btn {
        background-color: #353535;
        border: none;
        color: white;
        opacity: 0.5;
        padding-left: 10px;
        padding-top: 8px;
        cursor: pointer;
    }

    .notes-list {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 6px;
        padding-top: 8px;
    }

    .note-item {
        display: flex;
        width: 240px;
        gap: 4px;
    }

    .note-btn {
        flex: 1;
        padding: 8px 12px;
        background: #252525;
        color: #e0e0e0;
        border: 1px solid #444;
        border-radius: 4px;
        font-size: 0.9rem;
        text-align: left;
        cursor: pointer;
        transition: background 0.15s ease, border-color 0.15s ease;
        /* Truncate long titles with ellipsis */
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .note-btn:hover,
    .note-btn:active {
        background: #4a4a4a;
        color: white;
    }

    .delete-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        background: #252525;
        color: #888;
        border: 1px solid #444;
        border-radius: 4px;
        padding: 8px;
        line-height: 0;
        cursor: pointer;
        transition: background 0.15s ease, color 0.15s ease;
    }

    .delete-btn:hover {
        background: #8b0000;
        color: white;
        border-color: #a00000;
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
        flex-wrap: wrap;
        align-content: flex-start;
    }

    .toolbar {
        width: 100%;
        height: 50px;
        display: flex;
        gap: 8px;
        padding: 12px 16px;
        background-color: #1e1e1e;
        box-sizing: border-box;
    }

    .title-input {
        flex: 1;
        background: #1e1e1e;
        border: none;
        color: white;
        padding: 6px 10px;
        border-radius: 4px;
        outline: none;
    }

    .note-input,
    .preview {
        width: 50%;
        height: calc(100vh - 50px);
        box-sizing: border-box;
        overflow-y: auto;
    }

    .note-input {
        flex: 1;
        background-color: #1e1e1e;
        color: white;
        outline: none;
        border: none;
        border-right: 1px solid #444;
        resize: none;
        padding: 16px;
        font-family: inherit;
        font-size: 1rem;
    }

    .preview {
        background-color: #1e1e1e;
        padding: 16px;
        color: white;
    }
</style>