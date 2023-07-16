// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use itunes_xml::{parse_itunes_xml, Library, Track};
use rusqlite::{Connection, Result};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::State;
use types::QueryParams;

struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

#[tauri::command]
fn parse_itunes_xml_command(path: &str, app_state: State<AppState>) -> Result<(), String> {
    println!("{:?}", path);
    let library = parse_itunes_xml(path).map_err(|err| err.to_string())?;
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )
    .map_err(|err| err.to_string())?;
    //*app_state.library.lock().map_err(|err| err.to_string())? = library;
    Ok(())
}

#[tauri::command]
fn fetch_tracks_command(
    query: QueryParams,
    app_state: State<AppState>,
) -> Result<Vec<Track>, String> {
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, data FROM person")
        .map_err(|err| err.to_string())?;
    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .map_err(|err| err.to_string())?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    // let library = app_state.library.lock().map_err(|err| err.to_string())?;
    let library = Library {
        tracks: HashMap::new(),
        metadata: HashMap::new(),
        playlists: HashMap::new(),
    };
    Ok(library
        .tracks
        .clone()
        .into_values()
        .take(query.limit)
        .collect())
}

// TODO Consider file access via tauri command alternative
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
//
//     Ok(())
// }

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    // Run backend
    tauri::async_runtime::spawn(backend::main());

    let conn = Connection::open_in_memory().expect("Database open failed");
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )
    .expect("Failed to create table");

    tauri::Builder::default()
        .manage(AppState {
            db: Arc::new(Mutex::new(conn)),
        })
        .invoke_handler(tauri::generate_handler![
            parse_itunes_xml_command,
            fetch_tracks_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
