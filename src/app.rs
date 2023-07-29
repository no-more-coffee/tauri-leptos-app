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

async fn fetch_tracks(title: Option<&str>) -> Result<Vec<Track>, String> {
    tauri::invoke(
        "fetch_tracks_command",
        &QueryParamsArgs {
            query: QueryParams { limit: 10, title },
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
                <TracksView/>
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
fn TracksView(cx: Scope) -> impl IntoView {
    let (status, set_status) = create_signal(cx, String::default());
    let (title_filter, set_title_filter) = create_signal(cx, String::default());

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

    let title_filter_view = move || {
        view! { cx,
            <input type="text"
                on:input=move |ev| {
                    // event_target_value is a Leptos helper function
                    // it functions the same way as event.target.value
                    // in JavaScript, but smooths out some of the typecasting
                    // necessary to make this work in Rust
                    set_title_filter.set(event_target_value(&ev));
                }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=title_filter.get()
            />
            <p>"Name is: " {title_filter}</p>
        }
    };

    view! { cx,
        <main class="container">
            <p><b>{ move || status.get() }</b></p>

            <span>
                <button on:click=on_pause>{"⏯️"}</button>
                <button on:click=on_stop>{"⏹️"}</button>
            </span>

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
                        {title_filter_view}
                    </th>
                    <th>{"Artist"}</th>
                    <th>{"BPM"}</th>
                </tr>

                <TracksComponent title_filter=title_filter/>
            </table>
        </main>
    }
}

#[component]
fn TracksComponent(cx: Scope, title_filter: ReadSignal<String>) -> impl IntoView {
    let async_data = create_resource(
        cx,
        move || title_filter.get(),
        |tfs| async move {
            let tf = match tfs.as_str() {
                "" => None,
                s => Some(s),
            };
            fetch_tracks(tf).await
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
            Err(e) => view! { cx, <p>"Error:" { e }</p> }.into_view(cx),
        },
    }
}

#[component]
fn TrackRow(cx: Scope, track: Track) -> impl IntoView {
    let location_opt = track.location.map(|l| l.replacen("file://", "", 1));

    let on_play_track = move |ev: MouseEvent, l: String| {
        ev.prevent_default();

        spawn_local(async move {
            match play_track(l).await {
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

    let track_play_element = match location_opt {
        Some(l) => view! { cx,
            <td>
                <button on:click = move |ev| on_play_track(ev, l.clone())>
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

#[component]
fn ControlledComponent(cx: Scope) -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! { cx,
        <input type="text"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name.set(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //
            // IMPORTANT: the `value` *attribute* only sets the
            // initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; I didn't invent it.
            // Other frameworks gloss this over; I think it's
            // more important to give you access to the browser
            // as it really works.
            //
            // tl;dr: use prop:value for form inputs
            prop:value=name.get()
        />
        <p>"Name is: " {name.get()}</p>
    }
}
