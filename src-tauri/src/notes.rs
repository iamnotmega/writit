use dirs::home_dir;
use std::fs;

#[tauri::command]
pub fn save_note(name: String, contents: String) -> Result<(), String> {
    // Construct home directory for saving the note (~/.writit)
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Create notes directory if it does not exist
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }

    // Add file extension
    let filename = format!("{}.md", name);

    // Build full path to the note's file
    let file_path = notes_dir.join(filename);

    // Save the note and its contents to the notes directory
    fs::write(file_path, contents).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_notes() -> Result<Vec<String>, String> {
    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Return an empty list if directory does not exist
    if !notes_dir.exists() {
        return Ok(Vec::new());
    }

    // Create a blank list to store note titles
    let mut notes = Vec::new();

    // Open the notes folder and loop through every file
    for entry in fs::read_dir(notes_dir).map_err(|e| e.to_string())? {
        // Convert the entry into a file path and then into a plain string
        let path = entry.map_err(|e| e.to_string())?.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        // Remove file extension if it's a Markdown file, otherwise ignore the entry
        if filename.ends_with(".md") {
            notes.push(filename.replace(".md", ""));
        }
    }

    // Return list of notes
    Ok(notes)
}

#[tauri::command]
pub fn read_note(name: String) -> Result<String, String> {
    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Build full file path
    let file_path = notes_dir.join(format!("{}.md", name));

    // Read the file and return its contents
    fs::read_to_string(file_path).map_err(|e| e.to_string())
}