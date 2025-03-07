use dioxus::prelude::*;

#[component]
pub fn OpenLoopCapture() -> Element {
    // Camera settings
    let mut frame_rate = use_signal(|| 30);
    let mut shutter_time = use_signal(|| 1000); // microseconds

    // Capture configuration
    let mut glass_start = use_signal(|| 0);
    let mut glass_end = use_signal(|| 5000);
    let mut peg_step = use_signal(|| 100);
    let mut layer_count = use_signal(|| 3);
    let mut surface_to_context_layer_minus2 = use_signal(|| 1000);
    let mut surface_to_context_layer_minus1 = use_signal(|| 2000);
    let mut surface_to_data = use_signal(|| 3000);
    let mut data_to_context = use_signal(|| 500);
    let mut layer_spacing = use_signal(|| 800);
    let mut surface_detect_threshold_min = use_signal(|| 0.2);
    let mut surface_detect_threshold_max = use_signal(|| 0.8);

    let handle_capture = move |_| {
        println!(
            "Starting capture with: frame_rate={}, shutter_time={}µs, glass_start={}, glass_end={}, peg_step={}, layer_count={}",
            frame_rate(),
            shutter_time(),
            glass_start(),
            glass_end(),
            peg_step(),
            layer_count()
        );
        // Add server call here
    };

    rsx! {
        div { id: "standard-view",
            h2 { "Open Loop Capture" }
            div { class: "settings-grid",
                // Camera Settings Card - Full Width
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
                    div { class: "setting-item",
                        label { "Shutter Time (µs)" }
                        input {
                            r#type: "number",
                            value: "{shutter_time}",
                            oninput: move |evt| shutter_time.set(evt.value().parse().unwrap_or(1000)),
                        }
                    }
                }
                // Capture Configuration Card - 2 Columns
                div { class: "settings-card",
                    h3 { "Capture Configuration" }
                    div { class: "settings-grid-2col",
                        div { class: "setting-item",
                            label { "Glass Start (nm)" }
                            input {
                                r#type: "number",
                                value: "{glass_start}",
                                oninput: move |evt| glass_start.set(evt.value().parse().unwrap_or(0)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Glass End (nm)" }
                            input {
                                r#type: "number",
                                value: "{glass_end}",
                                oninput: move |evt| glass_end.set(evt.value().parse().unwrap_or(5000)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Peg Step (nm)" }
                            input {
                                r#type: "number",
                                value: "{peg_step}",
                                oninput: move |evt| peg_step.set(evt.value().parse().unwrap_or(100)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Layer Count" }
                            input {
                                r#type: "number",
                                value: "{layer_count}",
                                oninput: move |evt| layer_count.set(evt.value().parse().unwrap_or(3)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Surface to Context Layer -2 (nm)" }
                            input {
                                r#type: "number",
                                value: "{surface_to_context_layer_minus2}",
                                oninput: move |evt| surface_to_context_layer_minus2.set(evt.value().parse().unwrap_or(1000)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Surface to Context Layer -1 (nm)" }
                            input {
                                r#type: "number",
                                value: "{surface_to_context_layer_minus1}",
                                oninput: move |evt| surface_to_context_layer_minus1.set(evt.value().parse().unwrap_or(2000)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Surface to Data (nm)" }
                            input {
                                r#type: "number",
                                value: "{surface_to_data}",
                                oninput: move |evt| surface_to_data.set(evt.value().parse().unwrap_or(3000)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Data to Context (nm)" }
                            input {
                                r#type: "number",
                                value: "{data_to_context}",
                                oninput: move |evt| data_to_context.set(evt.value().parse().unwrap_or(500)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Layer Spacing (nm)" }
                            input {
                                r#type: "number",
                                value: "{layer_spacing}",
                                oninput: move |evt| layer_spacing.set(evt.value().parse().unwrap_or(800)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Surface Detect Threshold Min" }
                            input {
                                r#type: "number",
                                step: "0.01",
                                min: "0",
                                max: "1",
                                value: "{surface_detect_threshold_min}",
                                oninput: move |evt| surface_detect_threshold_min.set(evt.value().parse().unwrap_or(0.2)),
                            }
                        }
                        div { class: "setting-item",
                            label { "Surface Detect Threshold Max" }
                            input {
                                r#type: "number",
                                step: "0.01",
                                min: "0",
                                max: "1",
                                value: "{surface_detect_threshold_max}",
                                oninput: move |evt| surface_detect_threshold_max.set(evt.value().parse().unwrap_or(0.8)),
                            }
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
