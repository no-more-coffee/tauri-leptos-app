use itunes_xml::{Playlist, Track};
use leptos::ev::MouseEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri_sys::dialog::FileDialogBuilder;
use tauri_sys::tauri;
use types::QueryParams;

#[derive(Serialize)]
struct ParseCommandArgs<'a> {
    path: &'a str,
}

#[derive(Serialize)]
struct QueryParamsArgs {
    query: QueryParams,
}

async fn pick_file() -> Result<Option<PathBuf>, String> {
    FileDialogBuilder::new()
        .set_title("Select a file to mark this test as passing")
        .pick_file()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct NoArgs;

async fn fetch_tracks() -> Result<Vec<Track>, String> {
    tauri::invoke(
        "fetch_tracks_command",
        &QueryParamsArgs {
            query: QueryParams { limit: 10 },
        },
    )
    .await
    .map_err(|e| e.to_string())
}

// TODO Fix u64 id to str conversion
#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub tracks: HashMap<String, Track>,
    pub playlists: HashMap<String, Playlist>,
}

async fn parse_itunes_xml(lib_path: String) -> Result<(), String> {
    tauri::invoke(
        "parse_itunes_xml_command",
        &ParseCommandArgs { path: &lib_path },
    )
    .await
    .map_err(|e| e.to_string())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (status, set_status) = create_signal(cx, String::default());
    let (tracks, set_tracks) = create_signal(cx, Vec::new());

    let button_click = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match pick_file().await {
                Ok(Some::<PathBuf>(f)) => {
                    let picked_file = f.to_string_lossy().to_string();
                    set_status.set(picked_file.clone());

                    spawn_local(async move {
                        match parse_itunes_xml(picked_file).await {
                            Ok(_) => {
                                set_status.set("Library loaded".to_string());

                                spawn_local(async move {
                                    match fetch_tracks().await {
                                        Ok(tracks) => {
                                            set_tracks.set(tracks);
                                        }
                                        Err(e) => set_status.set(e),
                                    };
                                });
                            }
                            Err(e) => set_status.set(e),
                        };
                    });
                }
                Ok(None) => set_status.set(String::default()),
                Err(e) => set_status.set(e),
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
            <div class="btn-group">
                <button on:click=button_click>{"Choose Library"}</button>
                <p><b>{ move || status.get() }</b></p>
            </div>

            {tracks_table}
        </main>
    }
}
