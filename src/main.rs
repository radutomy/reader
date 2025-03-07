mod ui {
    pub mod root;
    pub mod sidebar;

    #[path = "open-capture.rs"]
    pub mod open_capture;
}

use ui::root::App;

fn main() {
    dioxus::launch(App);
}
