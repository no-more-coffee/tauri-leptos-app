// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use itunes_xml::{parse_itunes_xml, Library, Track};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::State;
use types::QueryParams;

struct AppState {
    library: Arc<Mutex<Library>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn parse_itunes_xml_command(path: &str, app_state: State<AppState>) -> Result<(), String> {
    println!("{:?}", path);
    // println!("{:?}", app_state.file_path.clone());

    let library = parse_itunes_xml(path).map_err(|err| err.to_string())?;
    *app_state.library.lock().map_err(|err| err.to_string())? = library;
    Ok(())
}

#[tauri::command]
fn fetch_tracks_command(query: QueryParams, app_state: State<AppState>) -> Result<Vec<Track>, String> {
    let library = app_state.library.lock().map_err(|err| err.to_string())?;
    Ok(library.tracks.clone().into_values().take(query.limit).collect())
}

#[tauri::command]
fn fetch_library_command(app_state: State<AppState>) -> Result<Library, String> {
    let library = app_state.library.lock().map_err(|err| err.to_string())?;
    Ok(library.clone())
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
    tauri::async_runtime::spawn(backend::main());

    let app_state = AppState {
        library: Arc::new(Mutex::new(Library {
            tracks: HashMap::new(),
            playlists: HashMap::new(),
        })),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            parse_itunes_xml_command,
            fetch_library_command,
            fetch_tracks_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
