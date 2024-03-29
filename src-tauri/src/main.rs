// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use rodio::{Decoder, OutputStream, Sink};
use rusqlite::{params_from_iter, Connection, Result};
use tauri::State;
use url::Url;

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
    app_state.sink.stop();

    let file_url = Url::parse(path).map_err(|err| err.to_string())?;
    let path_buf = file_url
        .to_file_path()
        .map_err(|_| "Failed to parse location")?;
    let file = File::open(path_buf).map_err(|err| err.to_string())?;
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
            );",
            (id, &track.name, &track.artist, &track.bpm, &track.location),
        )
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn is_library_loaded_command(
    app_state: State<AppState>,
) -> Result<bool, String> {
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=(?);").map_err(|err| err.to_string())?;
    let mut rows = stmt.query(["tracks"]).map_err(|err| err.to_string())?;
    Ok(rows.next().map_err(|err| err.to_string())?.is_some())
}

#[tauri::command]
fn fetch_tracks_command(
    query: QueryParams,
    app_state: State<AppState>,
) -> Result<Vec<Track>, String> {
    let conn = app_state.db.lock().map_err(|err| err.to_string())?;

    let mut query_parts = vec![];
    let mut params: Vec<String> = vec![];

    if let Some(title) = query.title {
        query_parts.push("(LOWER( name ) LIKE '%' || (?) || '%')");
        params.push(title.split_whitespace().collect::<Vec<&str>>().join("%"));
    };

    if let Some(artist) = query.artist {
        query_parts.push("(LOWER( artist ) LIKE '%' || (?) || '%')");
        params.push(artist.split_whitespace().collect::<Vec<&str>>().join("%"));
    };

    if let Some(bpm) = query.bpm_min {
        query_parts.push("( bpm >= (?) )");
        params.push(bpm.to_string());
    };

    if let Some(bpm) = query.bpm_max {
        query_parts.push("( bpm <= (?) )");
        params.push(bpm.to_string());
    };

    if let Some(location) = query.location {
        query_parts.push("(LOWER( location ) LIKE '%' || (?) || '%')");
        params.push(location.split_whitespace().collect::<Vec<&str>>().join("%"));
    };

    let wheres = match query_parts.is_empty() {
        true => "".to_string(),
        false => format!("WHERE {}", query_parts.join(" AND ")),
    };

    let full_query = format!("SELECT * FROM tracks {} LIMIT (?);", wheres);
    params.push(query.limit.to_string());

    println!("{full_query:?}, {params:?}");
    let mut statement = conn
        .prepare(full_query.as_str())
        .map_err(|err| err.to_string())?;
    let library_iter = statement
        .query_map(params_from_iter(params.iter()), |row| {
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
    // Open DB
    let conn = Connection::open("db.sqlite").expect("Database open failed");

    // Open sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    tauri::Builder::default()
        .manage(AppState {
            db: Arc::new(Mutex::new(conn)),
            sink: Arc::new(sink),
        })
        .invoke_handler(tauri::generate_handler![
            is_library_loaded_command,
            parse_itunes_xml_command,
            fetch_tracks_command,
            play_track_command,
            pause_command,
            stop_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
