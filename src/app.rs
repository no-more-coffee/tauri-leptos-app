// use itunes_xml::{Library, Track};
use leptos::*;
use leptos::{ev::MouseEvent, leptos_dom::ev::SubmitEvent};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, js_sys::JsString>;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// TODO Fix u64 id to str conversion
#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub tracks: HashMap<String, Track>,
    pub playlists: HashMap<String, Playlist>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: u64,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (message, set_message) = create_signal(cx, String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        log!("{:?}", v);
        set_name.set(v);
    };
    let button_click = move |ev: MouseEvent| {
        ev.prevent_default();
        spawn_local(async move {
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            log!("Before");
            let args = JsValue::default();
            log!("{:?}", invoke("save_file_path", args).await);
            log!("After");
            // let msg = match invoke2("save_file_path").await {
            //     Ok(ok) => {
            //         format!("{:?}", ok)
            //     }
            //     Err(e) => e.as_string().unwrap_or("Failed to greet".to_string()),
            // };
            // set_message.set(msg);
        });
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args =
                to_value(&GreetArgs { name: &name.get() }).expect("Failed to serialize args");
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

            // JsValue(Object({"playlists":{"1":{"id":1}},"tracks":{"5994":{"id":5994}}}))
            // let res = Ok(Library { tracks: {"5994": Track { id: 5994 }}, playlists: {"1": Playlist { id: 1 }} })
            // log!("{:?}", res);
            let msg = match invoke("greet", args).await.map(from_value::<Library>) {
                Ok(Ok(library)) => {
                    format!("{:?}", library.tracks)
                }
                Ok(Err(err)) => {
                    format!("{:?}", err)
                }
                Err(e) => e.as_string().unwrap_or("Failed to greet".to_string()),
            };
            set_message.set(msg);
        });
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

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <button on:click=button_click>{"Press"}</button>

            <p><b>{ move || message.get() }</b></p>

            <table>
                <tr>
                    <th>{"Track ID"}</th>
                    <th>{"Name"}</th>
                    <th>{"Artist"}</th>
                </tr>
                <tr>
                    <td>5994</td>
                    <td>{"Cross my heart"}</td>
                    <td>{"Diana"}</td>
                </tr>
                <tr>
                    <td>5994</td>
                    <td>{"Each and everyone"}</td>
                    <td>{"Vladimir"}</td>
                </tr>
                <tr>
                    <td>5994</td>
                    <td>{"Each and everyone"}</td>
                    <td>{"EBTG"}</td>
                </tr>
            </table>
        </main>
    }
}
