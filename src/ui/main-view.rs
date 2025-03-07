use crate::ui::sidebar::Sidebar;
use crate::ui::open_capture::OpenLoopCapture;
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

#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
