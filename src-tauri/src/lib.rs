mod notes;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            notes::save_note, notes::get_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
