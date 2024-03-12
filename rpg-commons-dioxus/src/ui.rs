use dioxus::core::{Element, Scope};
use dioxus::core_macro::{component, Props, render};
use dioxus::hooks::use_shared_state;
use dioxus_html as dioxus_elements;
use rpg_core::context::AppContext;

#[component]
pub fn AudioPlayButton(cx: Scope, track: String) -> Element {
    let app_context = use_shared_state::<AppContext>(cx);

    render!(div {
        button {
            onclick: move |_| {
                let audio = app_context.unwrap().read().audio.clone(); // TODO: custom use_audio hook
                let track = track.clone();

                cx.spawn(async move {
                    audio
                        .play(track)
                        .await
                        .unwrap();
                });
            },
            "▶"
        }
    })
}
