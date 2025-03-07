use dioxus::prelude::*;

#[component]
pub fn OpenLoopCapture() -> Element {
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
        div { id: "standard-view",
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
