<aside class="sidebar">
    <h3 id="sidebar-heading">Notes</h3>
</aside>

<textarea class="note-input"
placeholder="Write a note..."></textarea>

<style>
    :global(html, body) {
        background-color: #1e1e1e;
        color: white;
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

    .note-input {
        position: absolute;
        margin-left: 260px;
        width: calc(100% - 260px);
        height: 100vh;
        background-color: #1e1e1e;
        color: white;
        outline: none;
        border: none;
        resize: none;
        padding: 16px;
    }

    #sidebar-heading {
        padding-left: 8px; 
    }
</style>