use dioxus::prelude::*;

#[component]
pub fn SystemSettings() -> Element {
    let mut ip_address = use_signal(|| String::from("127.0.0.1"));
    let mut port = use_signal(|| 8080);

    let handle_save = move |_| {
        println!(
            "Saving system settings: IP Address: {}, Port: {}",
            ip_address(),
            port()
        );
        // Add server call here to save settings
    };

    rsx! {
        div { id: "standard-view",
            div { class: "settings-grid",
                div { class: "settings-card",
                    h2 { "Network Configuration" }
                    div { class: "setting-item",
                        label { "IP Address" }
                        input {
                            r#type: "text",
                            value: "{ip_address}",
                            oninput: move |evt| ip_address.set(evt.value().to_string()),
                        }
                    }
                    div { class: "setting-item",
                        label { "Port" }
                        input {
                            r#type: "number",
                            value: "{port}",
                            oninput: move |evt| port.set(evt.value().parse().unwrap_or(8080)),
                        }
                    }
                }
            }
            div { class: "action-button-container",
                button { class: "action-button", onclick: handle_save,
                    span { class: "button-icon", "💾" }
                    "Save Settings"
                }
            }
        }
    }
}
