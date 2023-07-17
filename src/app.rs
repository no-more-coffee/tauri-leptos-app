use itunes_xml::Track;
use leptos::ev::MouseEvent;
use leptos::*;
use serde::Serialize;
use std::path::PathBuf;
use tauri_sys::dialog::FileDialogBuilder;
use tauri_sys::tauri;
use types::QueryParams;

async fn pick_file() -> Result<Option<PathBuf>, String> {
    FileDialogBuilder::new()
        .set_title("Select a file to mark this test as passing")
        .pick_file()
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct ParseCommandArgs<'a> {
    path: &'a str,
}

async fn parse_itunes_xml(lib_path: String) -> Result<(), String> {
    tauri::invoke(
        "parse_itunes_xml_command",
        &ParseCommandArgs { path: &lib_path },
    )
    .await
    .map_err(|e| e.to_string())
}

async fn play_track(lib_path: String) -> Result<(), String> {
    tauri::invoke("play_track_command", &ParseCommandArgs { path: &lib_path })
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct QueryParamsArgs {
    query: QueryParams,
}

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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (status, set_status) = create_signal(cx, String::default());
    let (tracks, set_tracks) = create_signal(cx, Vec::new());

    let choose_file = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match pick_file().await {
                Ok(Some::<PathBuf>(f)) => {
                    let picked_file = f.to_string_lossy().to_string();
                    set_status.set("Loading library file...".to_string());

                    spawn_local(async move {
                        match parse_itunes_xml(picked_file).await {
                            Ok(_) => {
                                set_status.set("Loading tracks...".to_string());

                                spawn_local(async move {
                                    match fetch_tracks().await {
                                        Ok(tracks) => {
                                            set_tracks.set(tracks);
                                            set_status.set("Loaded tracks".to_string());
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
    let on_play_track = move |ev: MouseEvent, l: String| {
        ev.prevent_default();

        spawn_local(async move {
            match play_track(l).await {
                Ok(_) => set_status.set("Playing".to_string()),
                Err(e) => set_status.set(e),
            }
        })
    };

    let track_row = move |track: Track| {
        let location_opt = track.location.map(|l| l.replacen("file://", "", 1));

        let track_play_element = match location_opt {
            Some(l) => view! { cx,
            <td>
                <button on:click = move |ev| on_play_track(ev, l.clone())>{"Play"}
                </button>
                </td>
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
            <p><b>{ move || status.get() }</b></p>

            <button on:click=choose_file>{"Choose Library"}</button>

            { move || (!tracks.get().is_empty()).then( {tracks_table } ) }
        </main>
    }
}
