use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }
		Hero {}
		Echo {}
	}
}

#[component]
pub fn Hero() -> Element {
    rsx! {
		div { id: "hero",
			div { id: "links" }
		}
	}
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    rsx! {
		div { id: "echo",
			h4 { "ServerFn Echo" }
			input {
				placeholder: "Type here to echo...",
				oninput: move |event| async move {
				    let data = echo_server(event.value()).await.unwrap();
				    response.set(data);
				},
			}
			if !response().is_empty() {
				p {
					"Server echoed: "
					i { "{response}" }
				}
			}
		}
	}
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
