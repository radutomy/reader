mod ui {
    pub mod root;
    pub mod sidebar;

    #[path = "open-capture.rs"]
    pub mod open_capture;
    
    #[path = "system-settings.rs"]
    pub mod system_settings;
}

use ui::root::App;

fn main() {
    dioxus::launch(App);
}
