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

            <TracksTable/>
        </div>

        <div class="side">
            <div class="queue">
                <SidePanel/>
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
fn TracksTable() -> impl IntoView {
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
                <th>{"Play"}</th>
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
            />
        </table>
    }
}

#[component]
fn TracksComponent(state: RwSignal<State>) -> impl IntoView {
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
        view! { <TrackRow track=track/> }
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
fn TrackRow(track: Track) -> impl IntoView {
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

    view! {
        <tr>
            {track_play_element}
            <td>{track.id}</td>
            <td>{track.name}</td>
            <td>{track.artist}</td>
            <td>{track.bpm}</td>
            <td>{track.location}</td>
        </tr>
    }
}

#[component]
fn SidePanel() -> impl IntoView {
    let (items, set_items) = create_signal(5);

    let track_row = move |track: u64| {
        view! { <div class="queue-item">Hello { track } </div> }
    };

    (0..items.get()).map(track_row).collect_view().into_view()
}

/*
At src/app.rs:407:15, you access a signal or memo (defined at src/app.rs:401:30) outside a reactive tracking context. This might mean your app is not responding to changes in signal values in the way you expect.

1. If this is inside a `view!` macro, make sure you are passing a function, not a value.
  ❌ NO  <p>{x.get() * 2}</p>
  ✅ YES <p>{move || x.get() * 2}</p>

2. If it’s in the body of a component, try wrapping this access in a closure:
  ❌ NO  let y = x.get() * 2
  ✅ YES let y = move || x.get() * 2.

3. If you’re *trying* to access the value without tracking, use `.get_untracked()` or `.with_untracked()` instead.
 */
