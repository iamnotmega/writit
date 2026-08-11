use dirs::home_dir;
use std::fs;

#[tauri::command]
pub fn save_note(name: String, contents: String) -> Result<(), String> {
    // Construct home directory for saving the note (~/.writit)
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Create notes directory if it does not exist
    fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;

    // Add file extension
    let filename = format!("{}.md", name);

    // Build full path to the note's file
    let file_path = notes_dir.join(filename);

    // Save the note and its contents to the notes directory
    fs::write(file_path, contents).map_err(|e| e.to_string())?;

    Ok(())
}