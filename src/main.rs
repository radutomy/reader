mod ui {
    #[path = "main-view.rs"]
    pub mod main_view;
}

use ui::main_view::App;

fn main() {
    dioxus::launch(App);
}
