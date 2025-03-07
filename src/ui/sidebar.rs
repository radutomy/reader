use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
		div { id: "sidebar",
			h1 { id: "app-title", "Reader Control" }
			div { id: "nav-links",
				button { class: "nav-button active",
					span { class: "nav-icon", "☰" }
					span { class: "nav-text", "Open Loop Capture" }
				}
				div { class: "spacer" }
				button { class: "nav-button",
					span { class: "nav-icon", "⛭" }
					span { class: "nav-text", "System Settings" }
				}
				button { class: "nav-button",
					span { class: "nav-icon", ">_" }
					span { class: "nav-text", "Logs" }
				}
			}
		}
	}
}
