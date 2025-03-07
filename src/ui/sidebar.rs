use dioxus::prelude::*;

#[component]
pub fn Sidebar(
    active_page: Signal<String>,
    on_page_change: EventHandler<String>,
) -> Element {
    rsx! {
		div { id: "sidebar",
			h1 { id: "app-title", "Reader Control" }
			div { id: "nav-links",
				button { 
                    class: if active_page() == "openloop" { "nav-button active" } else { "nav-button" },
                    onclick: move |_| on_page_change.call("openloop".to_string()),
					span { class: "nav-icon", "☰" }
					span { class: "nav-text", "Open Loop Capture" }
				}
				div { class: "spacer" }
				button { 
                    class: if active_page() == "settings" { "nav-button active" } else { "nav-button" },
                    onclick: move |_| on_page_change.call("settings".to_string()),
					span { class: "nav-icon", "⛭" }
					span { class: "nav-text", "System Settings" }
				}
				button { 
                    class: if active_page() == "logs" { "nav-button active" } else { "nav-button" },
                    onclick: move |_| on_page_change.call("logs".to_string()),
					span { class: "nav-icon", ">_" }
					span { class: "nav-text", "Logs" }
				}
			}
		}
	}
}
