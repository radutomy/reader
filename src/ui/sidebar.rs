use crate::ui::root::View;
use dioxus::prelude::*;

#[component]
pub fn Sidebar(active_page: Signal<View>, on_page_change: EventHandler<View>) -> Element {
    rsx! {
        div { id: "sidebar",
            h1 { id: "app-title", "Reader Control" }
            div { id: "nav-links",
                button {
                    class: if active_page() == View::OpenLoop { "nav-button active" } else { "nav-button" },
                    onclick: move |_| on_page_change.call(View::OpenLoop),
                    span { class: "nav-icon", "☰" }
                    span { class: "nav-text", "Open Loop Capture" }
                }
                div { class: "spacer" }
                button {
                    class: if active_page() == View::Settings { "nav-button active" } else { "nav-button" },
                    onclick: move |_| on_page_change.call(View::Settings),
                    span { class: "nav-icon", "⛭" }
                    span { class: "nav-text", "System Settings" }
                }
                button {
                    class: if active_page() == View::Logs { "nav-button active" } else { "nav-button" },
                    onclick: move |_| on_page_change.call(View::Logs),
                    span { class: "nav-icon", ">_" }
                    span { class: "nav-text", "Logs" }
                }
            }
        }
    }
}
