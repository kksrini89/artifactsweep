use std::path::PathBuf;
use artifactsweep_core::{ JunkEntry, find_junk_paths, size_entries };

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn scan(path: String) -> Result<Vec<JunkEntry>, String> {
    let folder = PathBuf::from(&path);

    if !folder.exists() {
        return Err(format!("path does not exist: {path}"));
    }

    if !folder.is_dir() {
        return Err(format!("not a directory: {path}"));
    }

    let found = find_junk_paths(&folder);
    let entries = size_entries(&found);

    Ok(entries)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
