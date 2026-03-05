// Desktop entry point (requires dioxus-desktop feature)
#![allow(non_snake_case)]

use tracing::Level;
use besim_gui_dioxus::App;

#[cfg(not(target_arch = "wasm32"))]
use dioxus::LaunchBuilder;
#[cfg(not(target_arch = "wasm32"))]
use dioxus::desktop::{Config, WindowBuilder};
#[cfg(not(target_arch = "wasm32"))]
use rust_embed::RustEmbed;

#[cfg(not(target_arch = "wasm32"))]
#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to initialize logger");
    tracing::info!("Starting BeSMART GUI (Dioxus desktop version)");
    let config = Config::new()
        .with_window(WindowBuilder::new().with_title("BeSMART"))
        .with_custom_protocol("asset".to_string(), move |path, _data| {

            log::info!("Fetching asset: {}", path);
            // Remove leading slash and try to get the asset
            let asset_path = path.trim_start_matches('/');

            // Try to get the embedded asset
            if let Some(content) = Assets::get(asset_path) {
                let mime = mime_guess::from_path(asset_path)
                    .first_or_octet_stream()
                    .to_string();

                log::info!("Serving asset: {} ({} bytes)", asset_path, content.data.len());
                http::Response::builder()
                    .header("Content-Type", mime)
                    .status(200)
                    .body(content.data)
                    .unwrap()
            } else {
                http::Response::builder()
                    .status(404)
                    .body(std::borrow::Cow::Borrowed(&[] as &[u8]))
                    .unwrap()
            }
        });

    LaunchBuilder::new()
        .with_cfg(config)
        .launch(App);
}
