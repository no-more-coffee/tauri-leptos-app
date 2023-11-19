use std::path::PathBuf;

use leptos::ev::MouseEvent;
use leptos::logging::log;
use leptos::*;
use serde::Serialize;
use tauri_sys::dialog::FileDialogBuilder;
use tauri_sys::tauri;

use itunes_xml::Track;
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
    limit: usize,
    title: Option<&str>,
    artist: Option<&str>,
    bpm_min: Option<i64>,
    bpm_max: Option<i64>,
    location: Option<&str>,
) -> Result<Vec<Track>, String> {
    tauri::invoke(
        "fetch_tracks_command",
        &QueryParamsArgs {
            query: QueryParams {
                limit,
                title,
                artist,
                bpm_min,
                bpm_max,
                location,
            },
        },
    )
        .await
        .map_err(|e| e.to_string())
}

async fn fetch_library_loaded() -> Result<bool, String> {
    tauri::invoke(
        "is_library_loaded_command",
        &NoArgs {},
    )
        .await
        .map_err(|e| e.to_string())
}

#[component]
pub fn App() -> impl IntoView {
    let library_fetched = create_resource(
        || (),
        |_| async move { fetch_library_loaded().await },
    );

    let contents = move || match library_fetched.get() {
        None => view! {
            <p>"Loading..."</p>}.into_view(),
        Some(Err(e)) => view! {
            <p>"Error: " {e}</p>}.into_view(),
        Some(Ok(true)) => view! {
            <LibraryView/>}.into_view(),
        Some(Ok(false)) => view! {
            <ChooseLibrary library_fetched=library_fetched />}.into_view(),
    };

    view! {
        <main class="container">
            { contents }
        </main>
    }
}

#[component]
fn ChooseLibrary(library_fetched: Resource<(), Result<bool, String>>) -> impl IntoView {
    let (status, set_status) = create_signal(String::default());

    let choose_file = move |ev: MouseEvent| {
        ev.prevent_default();

        spawn_local(async move {
            match pick_file().await {
                Ok(Some::<PathBuf>(f)) => {
                    let picked_file = f.to_string_lossy().to_string();
                    set_status.set("Loading library file...".to_string());

                    spawn_local(async move {
                        match parse_itunes_xml(picked_file).await {
                            Ok(_) => library_fetched.refetch(),
                            Err(e) => set_status.set(e),
                        };
                    });
                }
                Ok(None) => set_status.set(String::default()),
                Err(e) => set_status.set(e),
            };
        });
    };

    view! {
        <div class="pick-file">
            <p class="status"><b>{ move || status.get() }</b></p>

            <button on:click=choose_file>{"Choose Library"}</button>
        </div>
    }
}

#[component]
fn LibraryView() -> impl IntoView {
    let (status, set_status) = create_signal(String::default());
    let (queue, set_queue) = create_signal(Vec::<Track>::default());

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

    view! {
        <div class="main">
            <p class="status"><b>{ move || status.get() }</b></p>

            <span>
                <button on:click=on_pause>{"⏯️"}</button>
                <button on:click=on_stop>{"⏹️"}</button>
            </span>

            <TracksTable set_queue=set_queue/>
        </div>

        <div class="side">
            <div class="queue">
                <SidePanel queue=queue set_queue=set_queue/>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
struct State {
    limit: String,
    title: String,
    artist: String,
    bpm_min: String,
    bpm_max: String,
    location: String,
}

#[component]
fn TracksTable(set_queue: WriteSignal<Vec<Track>>) -> impl IntoView {
    let state = create_rw_signal(State::default());
    let (title_filter, set_title_filter) = create_slice(
        state,
        |state| state.title.clone(),
        |state, v| state.title = v,
    );
    let (artist_filter, set_artist_filter) = create_slice(
        state,
        |state| state.artist.clone(),
        |state, v| state.artist = v,
    );
    let (bpm_min_filter, set_bpm_min_filter) = create_slice(
        state,
        |state| state.bpm_min.clone(),
        |state, v| state.bpm_min = v,
    );
    let (bpm_max_filter, set_bpm_max_filter) = create_slice(
        state,
        |state| state.bpm_max.clone(),
        |state, v| state.bpm_max = v,
    );
    let (location_filter, set_location_filter) = create_slice(
        state,
        |state| state.location.clone(),
        |state, v| state.location = v,
    );

    view! {
        <table>
            <tr>
                <th>{"Controls"}</th>
                <th>{"Track ID"}</th>
                <th>{"Name"}</th>
                <th>{"Artist"}</th>
                <th>{"BPM"}</th>
                <th>{"Location"}</th>
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
                    <input type="number" min="1" max="500"
                        on:input=move |ev| {
                            set_bpm_min_filter.set(event_target_value(&ev));
                        }
                        prop:value={move || bpm_min_filter.get()}
                    />
                    <input type="number" min="1" max="500"
                        on:input=move |ev| {
                            set_bpm_max_filter.set(event_target_value(&ev));
                        }
                        prop:value={move || bpm_max_filter.get()}
                    />
                </th>
                <th>
                    <input type="text"
                        on:input=move |ev| {
                            set_location_filter.set(event_target_value(&ev));
                        }
                        prop:value={move || location_filter.get()}
                    />
                </th>
             </tr>

            <TracksComponent
                state
                set_queue
            />
        </table>
    }
}

#[component]
fn TracksComponent(state: RwSignal<State>, set_queue: WriteSignal<Vec<Track>>) -> impl IntoView {
    let async_data = create_resource(
        move || state.get(),
        |value| async move {
            let title = match value.title.as_str() {
                "" => None,
                s => Some(s),
            };
            let artist = match value.artist.as_str() {
                "" => None,
                s => Some(s),
            };
            let bpm_min = value.bpm_min.parse::<i64>().ok();
            let bpm_max = value.bpm_max.parse::<i64>().ok();
            let location = match value.location.as_str() {
                "" => None,
                s => Some(s),
            };
            fetch_tracks(100, title, artist, bpm_min, bpm_max, location).await
        },
    );

    let track_row = move |track: Track| {
        view! { <TrackRow track set_queue/> }
    };

    move || match async_data.get() {
        None => view! { <p>"Loading..."</p> }.into_view(),
        Some(data) => match data {
            Ok(tracks) => tracks.into_iter().map(track_row).collect_view().into_view(),
            Err(e) => view! { <p>"Error: " {e}</p> }.into_view(),
        },
    }
}

#[component]
fn TrackRow(track: Track, set_queue: WriteSignal<Vec<Track>>) -> impl IntoView {
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

    let track_play_element = match track.location.clone() {
        Some(location) => view! {
            <td>
                <button on:click = move |ev| on_play_track(ev, location.clone())>
                    {"Play"}
                </button>
            </td>
        },
        None => view! {
            <td>{"Not found"}</td>
        },
    };

    let track_clone = track.clone();
    view! {
        <tr>
            // {track_play_element}
            <td>
                <button on:click=move |_|
                    set_queue.update(|queue|
                        queue.push(track_clone.clone())
                    )
                >
                    "Add"
                </button>
            </td>
            <td>{track.id}</td>
            <td>{track.name}</td>
            <td>{track.artist}</td>
            <td>{track.bpm}</td>
            <td>{track.location}</td>
        </tr>
    }
}

#[component]
fn SidePanel(queue: ReadSignal<Vec<Track>>, set_queue: WriteSignal<Vec<Track>>) -> impl IntoView {
    let track_view = |track: Track| view! {
        <div class="queue-item"><p>{ track.name }</p></div>
    };

    move || queue.get()
        .into_iter()
        .map(track_view)
        .collect_view()
}
