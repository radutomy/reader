mod ui {
    #[path = "main-view.rs"]
    pub mod main_view;

    #[path = "sidebar.rs"]
    pub mod sidebar;
    
    #[path = "open-capture.rs"]
    pub mod open_capture;
}

use ui::main_view::App;

fn main() {
    dioxus::launch(App);
}
