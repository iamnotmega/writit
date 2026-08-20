<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import { Trash2Icon, SquarePenIcon, FolderPlusIcon, SettingsIcon } from "@lucide/svelte";

    let saveTimer: ReturnType<typeof setTimeout> | undefined;
    let noteContent = $state("");
    let noteTitle = $state("Untitled");
    let activeNote = $state(""); // The note currently selected
    let notes = $state<string[]>([]); // List of saved notes
    let folders = $state<string[]>([]); // List of saved folders
    let selectedFolder = $state(""); // Folder where a new note will be created

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

    // Separate note name from folder name
    function splitNotePath(notePath: string) {
        // Position of the last / in the note path
        const separatorIndex = notePath.lastIndexOf("/");

        if (separatorIndex === -1) { // Runs if note is not in a folder
            return {
                folder: "", // Return no folder
                name: notePath // Return the note's name
            };
        }

        return { // Runs if note is in a folder
            folder: notePath.slice(0, separatorIndex), // Return the folder name, skipping the / after it
            name: notePath.slice(separatorIndex + 1) // Return the note's name 1 character after the /
        }
    }

    // Get the notes that are inside a folder
    function getNotesInFolder(folder: string) {
        return notes.filter((note) => {
            const { folder: noteFolder } = splitNotePath(note); // Split note and folder names
            return noteFolder === folder; // Return true if the note belongs to the folder
        });
    }

    // Get the notes that are not inside a folder
    function getRootNotes() {
        return notes.filter((note) => {
            const { folder } = splitNotePath(note);
            return folder === "";
        });
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

    // Attempt to load the list of folders
    async function loadFolders() {
        try {
            folders = await invoke("get_folders"); // Invoke backend command
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
    function createNewNote(folder: string = "") {
        clearTimeout(saveTimer); // Clear save timer

        // Reset note content and title
        activeNote = "";
        noteTitle = "Untitled";
        noteContent = "";

        // Remember which folder the note should be saved into
        selectedFolder = folder;

        // Clear note history
        undoStack = [];
        redoStack = [];
    }

    // Handle folder creation
    async function createNewFolder() {
        // Prompt the user for a folder name
        const name = prompt("Folder name:");

        // Stop if the name is empty
        if (!name || !name.trim()) return;

        // Attempt to create the folder
        try {
            await invoke("create_folder", {
                name: name.trim(),
            });

            // Refresh the list of folders
            loadFolders();
        } catch (err) {
            console.error("Failed to create folder:", err);
        }
    }


    // Handle note selection
    async function selectNote(name: string) {
        try {
        // Fetch note contents using backend
        const contents = await invoke<string>("read_note", { name });

        const { name: noteName } = splitNotePath(name);

        noteTitle = noteName;
        noteContent = contents;
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
            // Use the existing note's folder, or the selected folder for a new note
            const folder = activeNote
                ? splitNotePath(activeNote).folder
                : selectedFolder;

            // If the current note's title changed, rename the file
            if (activeNote) {
                const { name: activeName } = splitNotePath(activeNote);

                if (activeName !== title) {
                    await invoke("delete_note", { name: activeNote });
                }
            }

            await invoke("save_note", { // Invoke backend command, passing the note's title, contents and folder
                name: title,
                contents: noteContent,
                folder,
            });

            activeNote = folder ? `${folder}/${title}` : title; // Set active note as the full file path
            selectedFolder = "";
            await loadNotes(); // Refresh the list of notes after saving a new note
        } catch (err) { // Run on error
            console.log("Failed to save note:", err);
        }
    }

    // Handle deleting notes
    async function handleDelete(name: string) {
        if (!name || !name.trim()) return;

        // Ask the user to confirm before deleting the note
        if (confirm(`Delete note "${name}"?`)) {
            // Attempt to delete the note from disk
            try {
                // Invoke backend command to delete the note
                await invoke("delete_note", { name });

                // Reset note title and contents if the deleted note was the current note
                if (activeNote === name) {
                    noteTitle = "Untitled";
                    noteContent = "";
                    activeNote = "";
                    undoStack = [];
                    redoStack = [];
                }

                // Refresh note list
                await loadNotes();
            } catch (err) { // Print to console on error
                console.error("Failed to delete note:", err)
            }
        } else {
            return; // Abort deletion
        }
    }

    // Handle deleting folders
    async function handleDeleteFolder(name: string) {
        // Ask the user to confirm before deleting the folder
        const confirmation = prompt(
            `Type "${name}" to confirm deleting this folder and all notes inside it. Please note that this operation cannot be undone.`
        );

        // Stop if the user cancelled or typed the wrong name
        if (confirmation !== name) return;

        // Attempt to delete the folder
        try {
            await invoke("delete_folder", { name });

            // Refresh note and folder lists since deleting a folder also deletes the notes inside of it
            await loadFolders();
            await loadNotes();
        } catch (err) {
            console.error("Failed to delete folder:", err);
        }
    }

    // Load the list of notes and folders on startup
    onMount(() => {
        loadNotes();
        loadFolders();
    });
</script>

<aside class="sidebar">
    <button
        class="new-btn"
        aria-label="New note"
        title="New note"
        onclick={() => createNewNote()}
        >
        <SquarePenIcon />
        </button>

    <button
        class="new-btn"
        aria-label="New folder"
        title="New folder"
        onclick={() => createNewFolder()}
    >
        <FolderPlusIcon />
    </button>
    <div class="notes-list">
        <!-- Folders -->
        {#each folders as folder}
            <div class="folder">
                <div class="folder-header">
                    <span class="folder-name">{folder}</span>

                    <button
                        class="folder-new-btn"
                        onclick={() => createNewNote(folder)}
                        aria-label={`New note in ${folder}`}
                        title={`New note in ${folder}`}
                    >
                        <SquarePenIcon />
                    </button>

                    <button
                        class="folder-delete-btn"
                        onclick={() => handleDeleteFolder(folder)}
                        aria-label={`Delete ${folder} folder`}
                        title={`Delete ${folder} folder`}
                    >
                        <Trash2Icon />
                    </button>
                </div>

                <div class="folder-notes">
                    {#each getNotesInFolder(folder) as note}
                        <div class="note-item">
                            <button
                                class="note-btn {activeNote === note ? 'active' : ''}"
                                onclick={() => selectNote(note)}
                            >
                                {splitNotePath(note).name}
                            </button>

                            <button
                                class="delete-btn"
                                onclick={() => handleDelete(note)}
                                aria-label="Delete note"
                                title="Delete note"
                            >
                                <Trash2Icon />
                            </button>
                        </div>
                    {/each}
                </div>
            </div>
        {/each}

        <!-- Notes not belonging to a folder -->
        {#each getRootNotes() as note}
            <div class="note-item">
                <button
                    class="note-btn {activeNote === note ? 'active' : ''}"
                    onclick={() => selectNote(note)}
                >
                    {note}
                </button>

                <button
                    class="delete-btn"
                    onclick={() => handleDelete(note)}
                    aria-label="Delete note"
                    title="Delete note"
                >
                    <Trash2Icon />
                </button>
            </div>
        {/each}

        <!-- Empty state with no notes or folders -->
        {#if folders.length === 0 && getRootNotes().length === 0}
            <p class="empty">No notes yet...</p>
        {/if}
  </div>

  <a
      href="/settings"
      class="settings-btn"
      aria-label="Settings"
      title="Settings"
  >
      <SettingsIcon />
  </a>
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
        overflow: hidden;
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
        color: #666;
        padding-left: 10px;
        padding-top: 8px;
        cursor: pointer;
        transition: color 0.15s ease, color 0.15s ease;
    }

    .new-btn:hover {
        color: white;
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

    .settings-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        background: transparent;
        border: none;
        color: #888;
        padding: 8px;
        margin-top: 430px;
        margin-left: 220px;
        transition: color 0.15s ease, color 0.15s ease;
    }

    .settings-btn:hover {
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

    .folder {
        width: 240px;
        margin-bottom: 4px;
    }

    .folder-header {
        display: flex;
        align-items: center;
        width: 100%;
        padding: 4px 4px 4px 8px;
        box-sizing: border-box;
    }

    .folder-name {
        flex: 1;
        color: #aaa;
        font-size: 0.8rem;
        font-weight: bold;
    }

    .folder-delete-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        padding: 4px;
        background: transparent;
        color: #666;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .folder-delete-btn:hover {
        background: #8b0000;
        color: white;
    }

    .folder-new-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        padding: 4px;
        background: transparent;
        color: #666;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .folder-notes {
        display: flex;
        flex-direction: column;
        gap: 4px;
        padding-left: 8px;
    }
</style>
