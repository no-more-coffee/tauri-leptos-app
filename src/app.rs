use itunes_xml::{Playlist, Track};
use leptos::ev::MouseEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri_sys::dialog::FileDialogBuilder;
use tauri_sys::tauri;

#[derive(Serialize)]
struct ParseCommandArgs<'a> {
    path: &'a str,
}

#[derive(Serialize)]
struct NoArgs;

async fn parse_itunes_xml(lib_path: String) -> Result<String, String> {
    tauri::invoke(
        "parse_itunes_xml_command",
        &ParseCommandArgs { path: &lib_path },
    )
    .await
    .map_err(|e| e.to_string())
}

async fn fetch_library() -> Result<Library, String> {
    tauri::invoke("fetch_library_command", &NoArgs {})
        .await
        .map_err(|e| e.to_string())
}

// TODO Fix u64 id to str conversion
#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub tracks: HashMap<String, Track>,
    pub playlists: HashMap<String, Playlist>,
}

async fn pick_file() -> Result<Option<PathBuf>, String> {
    FileDialogBuilder::new()
        .set_title("Select a file to mark this test as passing")
        .pick_file()
        .await
        .map_err(|e| e.to_string())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // let (name, set_name) = create_signal(cx, String::default());
    let (lib_path, set_lib_path) = create_signal(cx, String::default());
    let (lib_loaded, set_lib_loaded) = create_signal(cx, String::default());
    let (tracks, set_tracks) = create_signal(cx, Vec::new());

    let button_click = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let picked_file = match pick_file().await {
                Ok(Some::<PathBuf>(f)) => f.to_string_lossy().to_string(),
                Ok(None) => String::default(),
                Err(e) => e,
            };
            set_lib_path.set(picked_file);
        });
    };

    // let submit = move |ev: SubmitEvent| {
    let submit = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if lib_path.get().is_empty() {
                return;
            }

            match parse_itunes_xml(lib_path.get()).await {
                Ok(lib_loaded) => {
                    log!("{:?}", lib_loaded);
                    set_lib_loaded.set(lib_loaded);
                }
                Err(e) => log!("{:?}", e),
            };
        });
    };

    let load = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if lib_loaded.get().is_empty() {
                return;
            }

            match fetch_library().await {
                Ok(library) => {
                    log!("{:?}", library);
                    set_tracks.set(library.tracks.into_values().collect());
                }
                Err(e) => log!("{:?}", e),
            };
        });
    };

    let track_row = move |track: Track| {
        let location_opt = track
            .location
            .map(|l| l.replacen("file://", "http://localhost:3000/files", 1));
        let track_play_element = match location_opt {
            Some(l) => view! { cx,
                    <td><audio
                        controls
                        preload="none"
                        src={l}>
                        "Cannot play the audio element"
                    </audio></td>
            },
            None => view! { cx,
                    <td>{"Not found"}</td>
            },
        };
        view! { cx,
            <tr>
                {track_play_element}
                <td>{track.id}</td>
                <td>{track.name}</td>
                <td>{track.artist}</td>
                <td>{track.bpm}</td>
            </tr>
        }
    };
    let tracks_table = move || {
        view! { cx,
            <table>
                <tr>
                    <th>{"Play"}</th>
                    <th>{"Track ID"}</th>
                    <th>{"Name"}</th>
                    <th>{"Artist"}</th>
                    <th>{"BPM"}</th>
                </tr>

                { tracks.get().into_iter()
                    .map(track_row)
                    .collect::<Vec<_>>()
                }
            </table>
        }
    };

    view! { cx,
        <main class="container">
            <button on:click=button_click>{"Choose Library"}</button>
            <button on:click=submit>{"Read Library"}</button>
            <button on:click=load>{"Load Tracks"}</button>

            <p><b>{ move || lib_path.get() }</b></p>
            <p><b>{ move || lib_loaded.get() }</b></p>

            {tracks_table}
        </main>
    }
}
