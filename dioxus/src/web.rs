// Web entry point (wasm)
#![allow(non_snake_case)]

use besim_gui_dioxus::App;
use tracing::Level;

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to initialize logger");
    tracing::info!("Starting BeSMART GUI (Dioxus web version)");
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
