// Desktop entry point (requires dioxus-desktop feature)
#![allow(non_snake_case)]

use tracing::Level;
use besim_gui_dioxus::App;

#[cfg(not(target_arch = "wasm32"))]
use dioxus::LaunchBuilder;
#[cfg(not(target_arch = "wasm32"))]
use dioxus::desktop::{Config, WindowBuilder};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to initialize logger");
    tracing::info!("Starting BeSMART GUI (Dioxus desktop version)");
    let config = Config::new()
        .with_window(WindowBuilder::new().with_title("BeSMART"));

    LaunchBuilder::new()
        .with_cfg(config)
        .launch(App);
}
