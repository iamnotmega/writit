use dirs::home_dir;
use std::fs;
use std::path::Path;

#[tauri::command]
fn validate_note_path(name: &str) -> Result<(), String> {
    // Split the note path into its individual components
    for component in Path::new(name).components() {
        match component {
            // Allow normal folder and note names
            std::path::Component::Normal(_) => {}

            // Reject everything else (such as ".." or absolute paths)
            _ => {
                return Err("Invalid note path".to_string());
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub fn save_note(name: String, contents: String, folder: String) -> Result<(), String> {
    // Check note name validity
    validate_note_path(&name)?;

    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Create notes directory if it does not exist
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }

    // Check if a folder for the note was provided
    let folder_dir = if folder.trim().is_empty() {
        notes_dir.clone() // Save directly to .writit
    } else {
        notes_dir.join(folder.trim()) // Add folder path on top of .writit
    };

    // Create folder if it doesn't exist already
    if !folder_dir.exists() {
        fs::create_dir_all(&folder_dir).map_err(|e| e.to_string())?;
    }

    // Add file extension
    let filename = format!("{}.md", name);

    // Build full path to the note's file
    let file_path = folder_dir.join(filename);

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
    for entry in fs::read_dir(&notes_dir).map_err(|e| e.to_string())? {
        // Store the file path of the entry
        let path = entry.map_err(|e| e.to_string())?.path();
        
        // Check if the entry is a file
        if path.is_file() {
            // Get the filename without the file path
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.ends_with(".md") { // Check if file has .md extension
                    notes.push(filename.trim_end_matches(".md").to_string()); // Add the file to the list of notes
                }
            }
        }

        // Check if the entry is a directory
        if path.is_dir() {
            // Get the name of the folder
            if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                // Loop through every file in the folder
                for folder_entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
                    // Get the full path of the entry inside the folder
                    let folder_path = folder_entry.map_err(|e| e.to_string())?.path();

                    // Check if the folder entry is a file
                    if folder_path.is_file() {
                        // Get the filename of the note
                        if let Some(filename) = folder_path.file_name().and_then(|n| n.to_str()) {
                            // Check if file has the .md extension
                            if filename.ends_with(".md") {
                                // Add the folder and note to the notes list
                                notes.push(format!(
                                    "{}/{}",
                                    folder_name,
                                    filename.trim_end_matches(".md")
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort notes alphabetically
    notes.sort();

    // Return list of notes
    Ok(notes)
}

#[tauri::command]
pub fn delete_note(name: String) -> Result<(), String> {
    // Check note name validity
    validate_note_path(&name)?;

    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Add file extension to note name
    let filename = format!("{}.md", name);

    // Build full path to the note file, including the folder if its in one
    let file_path = notes_dir.join(filename);

    // Delete the note from the disk
    fs::remove_file(file_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn read_note(name: String) -> Result<String, String> {
    // Check note name validity
    validate_note_path(&name)?;

    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Build full file path
    let file_path = notes_dir.join(format!("{}.md", name));

    // Read the file and return its contents
    fs::read_to_string(file_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_folder(name: String) -> Result<(), String> {
    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Remove newline character and spaces from folder name
    let folder_name = name.trim();

    // Throw error if folder name is empty
    if folder_name.is_empty() {
        return Err("Folder name cannot be empty".to_string());
    }

    if Path::new(folder_name).components().count() != 1 { // Throw error if folder name has 2 components (e.g School/Biology)
        return Err("Invalid folder name".to_string());
    }

    // Build folder path
    let folder_path = notes_dir.join(folder_name);

    // Create the folder on the drive and any needed subdirectories
    fs::create_dir_all(folder_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_folders() -> Result<Vec<String>, String> {
    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Create notes directory (and all needed subdirectories) if it doesn't exist
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }

    // New vector for storing the list of folders
    let mut folders = Vec::new();

    // Loop through every folder in the notes directory
    for entry in fs::read_dir(notes_dir).map_err(|e| e.to_string())? {
        // File path of the folder
        let path = entry.map_err(|e| e.to_string())?.path();

        // Only store the entry in the folders vector if it's a directory
        if path.is_dir() {
            // Get only the folder name, not the full path
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                folders.push(name.to_string()); // Add folder to the top of the folders vector
            }
        }
    }
    
    // Sort folders in alphabetical order
    folders.sort();

    // Return the list of folders
    Ok(folders)
}

#[tauri::command]
pub fn delete_folder(name: String) -> Result<(), String> {
    // Construct notes directory
    let home = home_dir().ok_or("Could not find home directory")?;
    let notes_dir = home.join(".writit");

    // Remove spaces and newline from the folder name
    let folder_name = name.trim();

    // Reject empty folder names
    if folder_name.is_empty() {
        return Err("Folder name cannot be empty".to_string());
    }

    // Only allow a single folder name
    if Path::new(folder_name).components().count() != 1 {
        return Err("Invalid folder name".to_string());
    }

    // Build the full path to the folder
    let folder_path = notes_dir.join(folder_name);

    // Delete the folder and its contents
    fs::remove_dir_all(folder_path).map_err(|e| e.to_string())?;

    Ok(())
}