mod ui {
    #[path = "main-view.rs"]
    pub mod main_view;

    #[path = "sidebar.rs"]
    pub mod sidebar;
}

use ui::main_view::App;

fn main() {
    dioxus::launch(App);
}
