// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use rodio::{Decoder, OutputStream, Sink};
use rusqlite::{Connection, Result};
use tauri::State;

use itunes_xml::{parse_itunes_xml, Track};
use types::QueryParams;

struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub sink: Arc<Sink>,
}

#[tauri::command]
fn pause_command(app_state: State<AppState>) -> Result<bool, String> {
    match app_state.sink.is_paused() {
        false => {
            app_state.sink.pause();
            Ok(true)
        }
        true => {
            app_state.sink.play();
            Ok(false)
        }
    }
}

#[tauri::command]
fn stop_command(app_state: State<AppState>) -> Result<(), String> {
    app_state.sink.stop();
    Ok(())
}

#[tauri::command]
fn play_track_command(path: &str, app_state: State<AppState>) -> Result<(), String> {
    let file = File::open(path).map_err(|err| err.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|err| err.to_string())?;
    app_state.sink.append(source);

    Ok(())
}

#[tauri::command]
fn parse_itunes_xml_command(path: &str, app_state: State<AppState>) -> Result<(), String> {
    println!("{:?}", path);
    let library = parse_itunes_xml(path).map_err(|err| err.to_string())?;
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;

    if conn.execute("DROP TABLE tracks", ()).is_ok() {
        println!("Existing table dropped");
    };

    conn.execute(
        "CREATE TABLE tracks (
            id          INTEGER PRIMARY KEY,
            name        TEXT,
            artist      TEXT,
            bpm         INTEGER,
            location    TEXT
        )",
        (), // empty list of parameters.
    )
    .map_err(|err| err.to_string())?;

    for (id, track) in &library.tracks {
        conn.execute(
            "INSERT INTO tracks (
                id,
                name,
                artist,
                bpm,
                location
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5
            )",
            (id, &track.name, &track.artist, &track.bpm, &track.location),
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn fetch_tracks_command(
    query: QueryParams,
    app_state: State<AppState>,
) -> Result<Vec<Track>, String> {
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;
    let mut statement = conn
        .prepare(format!("SELECT * FROM tracks LIMIT {}", query.limit).as_str())
        .map_err(|err| err.to_string())?;
    let library_iter = statement
        .query_map([], |row| {
            let track = Track {
                id: row.get(0)?,
                name: row.get(1)?,
                artist: row.get(2)?,
                bpm: row.get(3)?,
                location: row.get(4)?,
                ..Default::default()
            };
            Ok(track)
        })
        .map_err(|err| err.to_string())?;

    Ok(library_iter.map(|row| row.unwrap()).collect())
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

fn main() {
    // Run backend
    // tauri::async_runtime::spawn(backend::main());

    let conn = Connection::open_in_memory().expect("Database open failed");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    tauri::Builder::default()
        .manage(AppState {
            db: Arc::new(Mutex::new(conn)),
            sink: Arc::new(sink),
        })
        .invoke_handler(tauri::generate_handler![
            parse_itunes_xml_command,
            fetch_tracks_command,
            play_track_command,
            pause_command,
            stop_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
