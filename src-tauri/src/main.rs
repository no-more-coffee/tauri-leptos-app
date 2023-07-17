// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use itunes_xml::{parse_itunes_xml, Track};
use rodio::{source::Source, Decoder, OutputStream, Sink};
use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use tauri::State;
use types::QueryParams;

struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

#[tauri::command]
fn play_track_command(path: &str, app_state: State<AppState>) -> Result<(), String> {
    dbg!(&path);
    let (_stream, stream_handle) = OutputStream::try_default().map_err(|err| err.to_string())?;
    let sink = Sink::try_new(&stream_handle).map_err(|err| err.to_string())?;
    let file = File::open(path).map_err(|err| err.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|err| err.to_string())?;

    /*stream_handle
        .play_raw(source.convert_samples())
        .map_err(|err| err.to_string())?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    */
    sink.append(source);
    sink.sleep_until_end();
    dbg!("End");

    Ok(())
}

#[tauri::command]
fn parse_itunes_xml_command(path: &str, app_state: State<AppState>) -> Result<(), String> {
    println!("{:?}", path);
    let library = parse_itunes_xml(path).map_err(|err| err.to_string())?;
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;

    if let Ok(_) = conn.execute("DROP TABLE tracks", ()) {
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
            let mut track = Track::default();
            track.id = row.get(0)?;
            track.name = row.get(1)?;
            track.artist = row.get(2)?;
            track.bpm = row.get(3)?;
            track.location = row.get(4)?;
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
    tauri::async_runtime::spawn(backend::main());

    let conn = Connection::open_in_memory().expect("Database open failed");

    tauri::Builder::default()
        .manage(AppState {
            db: Arc::new(Mutex::new(conn)),
        })
        .invoke_handler(tauri::generate_handler![
            parse_itunes_xml_command,
            fetch_tracks_command,
            play_track_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
