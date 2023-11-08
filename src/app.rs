use std::path::PathBuf;

use leptos::ev::MouseEvent;
use leptos::*;
use serde::Serialize;
use tauri_sys::dialog::FileDialogBuilder;
use tauri_sys::tauri;

use itunes_xml::Track;
use tracing::field::debug;
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

#[derive(Serialize)]
struct PlayTrackArgs<'a> {
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

async fn play_track(lib_path: &str) -> Result<(), String> {
    tauri::invoke("play_track_command", &PlayTrackArgs { path: lib_path })
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct NoArgs {}

async fn pause() -> Result<bool, String> {
    tauri::invoke("pause_command", &NoArgs {})
        .await
        .map_err(|e| e.to_string())
}

async fn stop() -> Result<(), String> {
    tauri::invoke("stop_command", &NoArgs {})
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct QueryParamsArgs<'a> {
    query: QueryParams<'a>,
}

async fn fetch_tracks(
    title: Option<&str>,
    artist: Option<&str>,
    bpm: Option<i64>,
    location: Option<&str>,
) -> Result<Vec<Track>, String> {
    tauri::invoke(
        "fetch_tracks_command",
        &QueryParamsArgs {
            query: QueryParams {
                limit: 10,
                title,
                artist,
                bpm,
                location,
            },
        },
    )
    .await
    .map_err(|e| e.to_string())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (library_loaded, set_library_loaded) = create_signal(cx, false);

    view! { cx,
        <main class="container">
            <Show
                when=move || {library_loaded.get()}
                fallback=move |cx| view! { cx, <ChooseLibrary set_library_loaded=set_library_loaded/> }
            >
                <LibraryView/>
            </Show>
        </main>
    }
}

#[component]
fn ChooseLibrary(cx: Scope, set_library_loaded: WriteSignal<bool>) -> impl IntoView {
    let (status, set_status) = create_signal(cx, String::default());

    let choose_file = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match pick_file().await {
                Ok(Some::<PathBuf>(f)) => {
                    let picked_file = f.to_string_lossy().to_string();
                    set_status.set("Loading library file...".to_string());

                    spawn_local(async move {
                        match parse_itunes_xml(picked_file).await {
                            Ok(_) => set_library_loaded.set(true),
                            Err(e) => set_status.set(e),
                        };
                    });
                }
                Ok(None) => set_status.set(String::default()),
                Err(e) => set_status.set(e),
            };
        });
    };

    view! { cx,
        <main class="container">
            <p><b>{ move || status.get() }</b></p>

            <button on:click=choose_file>{"Choose Library"}</button>
        </main>
    }
}

#[component]
fn LibraryView(cx: Scope) -> impl IntoView {
    let (status, set_status) = create_signal(cx, String::default());

    let on_pause = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match pause().await {
                Ok(true) => set_status.set("Paused".to_string()),
                Ok(false) => set_status.set("Playing".to_string()),
                Err(e) => set_status.set(e),
            }
        })
    };

    let on_stop = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match stop().await {
                Ok(_) => set_status.set("Stopped".to_string()),
                Err(e) => set_status.set(e),
            }
        })
    };

    view! { cx,
        <main class="container">
            <p><b>{ move || status.get() }</b></p>

            <span>
                <button on:click=on_pause>{"⏯️"}</button>
                <button on:click=on_stop>{"⏹️"}</button>
            </span>

            <TracksTable/>
        </main>
    }
}

#[component]
fn TracksTable(cx: Scope) -> impl IntoView {
    let (title_filter, set_title_filter) = create_signal(cx, String::default());
    let (artist_filter, set_artist_filter) = create_signal(cx, String::default());
    let (bpm_filter, set_bpm_filter) = create_signal(cx, String::default());

    view! { cx,
        <table>
            <tr>
                <th>{"Play"}</th>
                <th>{"Track ID"}</th>
                <th>{"Name"}</th>
                <th>{"Artist"}</th>
                <th>{"BPM"}</th>
            </tr>

            <tr>
                <th></th>
                <th></th>
                <th>
                    <input type="text"
                        on:input=move |ev| {
                            set_title_filter.set(event_target_value(&ev));
                        }
                        prop:value={move || title_filter.get()}
                    />
                </th>
                <th>
                    <input type="text"
                        on:input=move |ev| {
                            set_artist_filter.set(event_target_value(&ev));
                        }
                        prop:value={move || artist_filter.get()}
                    />
                </th>
                <th>
                    <input type="text"
                        on:input=move |ev| {
                            set_bpm_filter.set(event_target_value(&ev));
                        }
                        prop:value={move || bpm_filter.get()}
                    />
                </th>
            </tr>

            <TracksComponent
                title_filter=title_filter
                artist_filter=artist_filter
                bpm_filter=bpm_filter
            />
        </table>
    }
}

#[component]
fn TracksComponent(
    cx: Scope,
    title_filter: ReadSignal<String>,
    artist_filter: ReadSignal<String>,
    bpm_filter: ReadSignal<String>,
) -> impl IntoView {
    // Add bpm conversion later
    debug(bpm_filter);
    debug(artist_filter);
    let async_data = create_resource(
        cx,
        move || title_filter.get(),
        |tfs| async move {
            let tf = match tfs.as_str() {
                "" => None,
                s => Some(s),
            };
            fetch_tracks(tf, None, None, None).await
        },
    );

    let track_row = move |track: Track| {
        view! { cx, <TrackRow track=track/> }
    };

    move || match async_data.read(cx) {
        None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
        Some(data) => match data {
            Ok(tracks) => tracks
                .into_iter()
                .map(track_row)
                .collect_view(cx)
                .into_view(cx),
            Err(e) => view! { cx, <p>"Error: " {e}</p> }.into_view(cx),
        },
    }
}

#[component]
fn TrackRow(cx: Scope, track: Track) -> impl IntoView {
    let on_play_track = move |ev: MouseEvent, location: String| {
        ev.prevent_default();

        spawn_local(async move {
            match play_track(&location).await {
                Ok(_) => {
                    // set_status.set("Playing".to_string())
                    log!("Should be playing");
                }
                Err(e) => {
                    log!("Play track error: {}", e);
                    // set_status.set(e)
                }
            }
        })
    };

    let track_play_element = match track.location {
        Some(location) => view! { cx,
            <td>
                <button on:click = move |ev| on_play_track(ev, location.clone())>
                    {"Play"}
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
}
