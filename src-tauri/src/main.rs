// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use itunes_xml::{parse_itunes_xml, Library, Track};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let Ok(library) = parse_itunes_xml("../itunes-xml/tests/fixtures/Export.xml") else {
        return "File not found".to_string()
    };
    format!(
        "Hello, {}! You've been greeted from Rust!. Tracks: {:?}",
        name,
        library.tracks.len()
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
