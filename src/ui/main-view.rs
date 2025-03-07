use crate::ui::sidebar::Sidebar;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		div { id: "app-container",
			Sidebar {}
			div { id: "main-content",
				div { id: "top-bar" }
				div { id: "content-area", OpenLoopCapture {} }
			}
		}
	}
}

#[component]
fn OpenLoopCapture() -> Element {
    let mut frame_rate = use_signal(|| 30);
    let mut glass_start = use_signal(|| 0);

    let handle_capture = move |_| {
        println!(
            "Starting capture with frame rate: {} and glass start: {}",
            frame_rate(),
            glass_start()
        );
        // Add server call here
    };

    rsx! {
		div { id: "openloop-capture",
			h2 { "Open Loop Capture" }
			div { class: "settings-grid",
				// Camera Settings Card
				div { class: "settings-card",
					h3 { "Camera Settings" }
					div { class: "setting-item",
						label { "Frame Rate (Hz)" }
						input {
							r#type: "number",
							value: "{frame_rate}",
							oninput: move |evt| frame_rate.set(evt.value().parse().unwrap_or(30)),
						}
					}
				}
				// Capture Configuration Card
				div { class: "settings-card",
					h3 { "Capture Configuration" }
					div { class: "setting-item",
						label { "Glass Start (nm)" }
						input {
							r#type: "number",
							value: "{glass_start}",
							oninput: move |evt| glass_start.set(evt.value().parse().unwrap_or(0)),
						}
					}
				}
			}
			div { class: "action-button-container",
				button { class: "action-button", onclick: handle_capture,
					span { class: "button-icon", "▶" }
					"Start Open Loop Capture"
				}
			}
		}
	}
}

#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
