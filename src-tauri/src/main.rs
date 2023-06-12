// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use itunes_xml::{parse_itunes_xml, Library};
use std::sync::{Arc, Mutex};
use tauri::api::dialog::FileDialogBuilder;
use tauri::State;

struct AppState {
    file_path: Arc<Mutex<String>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn parse_itunes_xml_command(path: &str, app_state: State<AppState>) -> Result<Library, String> {
    println!("{:?}", path);
    // println!("{:?}", app_state.file_path.clone());

    parse_itunes_xml(path).map_err(|err| err.to_string())
}

// #[tauri::command]
// fn save_file_path(app_state: State<AppState>) -> Result<(), String> {
//     let saved_file_path = app_state.file_path.clone();
//     FileDialogBuilder::default().pick_file(move |path_buf| {
//         // do something with the optional file path here
//         // the file path is `None` if the user closed the dialog
//         if let Some(file_path) = path_buf {
//             let mut state = saved_file_path.lock().unwrap();
//             *state = file_path.to_string_lossy().to_string();
//         }
//     });

//     Ok(())
// }

fn main() {
    let app_state = AppState {
        file_path: Arc::new(Mutex::new(String::new())),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![parse_itunes_xml_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
