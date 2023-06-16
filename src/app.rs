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

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     log!("{:?}", v);
    //     set_name.set(v);
    // };

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

    let tracks_table = move || {
        view! { cx,
            <table>
                <tr>
                    <th>{"Track ID"}</th>
                    <th>{"Name"}</th>
                    <th>{"Artist"}</th>
                    <th>{"BPM"}</th>
                </tr>

                { tracks.get().into_iter()
                    .map(|n| view! { cx,
                        <tr>
                            <td>{n.id}</td>
                            <td>{n.name}</td>
                            <td>{n.artist}</td>
                            <td>{n.bpm}</td>
                        </tr>
                    })
                    .collect::<Vec<_>>()
                }
            </table>
        }
    };

    view! { cx,
        <main class="container">
            // <div class="row">
            //     <a href="https://tauri.app" target="_blank">
            //         <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            //     </a>
            //     <a href="https://docs.rs/leptos/" target="_blank">
            //         <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
            //     </a>
            // </div>

            // <p>"Click on the Tauri and Leptos logos to learn more."</p>

            // <p>
            //     "Recommended IDE setup: "
            //     <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
            //     " + "
            //     <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
            //     " + "
            //     <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            // </p>

            // <form class="row" on:submit=greet>
            //     <input
            //         id="greet-input"
            //         placeholder="Enter a name..."
            //         on:input=update_name
            //     />
            //     <button type="submit">"Submit"</button>
            // </form>

            <button on:click=button_click>{"Press"}</button>
            <button on:click=submit>{"Submit"}</button>
            <button on:click=load>{"Load"}</button>
            // <audio
            //     controls
            //     src="/media/01 Feather (Feat. Cise Starr & Akin From Cyne).mp3">
            //         <a href="/media/01 Feather (Feat. Cise Starr & Akin From Cyne).mp3">
            //             {"Feather (Feat. Cise Starr & Akin From Cyne)"}
            //         </a>
            // </audio>

            <p><b>{ move || lib_path.get() }</b></p>
            <p><b>{ move || lib_loaded.get() }</b></p>

            {tracks_table}
        </main>
    }
}
