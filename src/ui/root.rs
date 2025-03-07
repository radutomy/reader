use crate::ui::sidebar::Sidebar;
use crate::ui::open_capture::OpenLoopCapture;
use crate::ui::system_settings::SystemSettings;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let mut active_page = use_signal(|| "openloop".to_string());

    rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		div { id: "app-container",
			Sidebar {
				active_page,
				on_page_change: move |page| active_page.set(page),
			}
			div { id: "main-content",
				div { id: "top-bar" }
				div { id: "content-area",
					{
					    match active_page().as_str() {
					        "openloop" => rsx! {
						OpenLoopCapture {}
					},
					        "settings" => rsx! {
						SystemSettings {}
					},
					        "logs" => rsx! {
						div { "Logs - Coming Soon" }
					},
					        _ => rsx! {
						OpenLoopCapture {}
					},
					    }
					}
				}
			}
		}
	}
}

#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
