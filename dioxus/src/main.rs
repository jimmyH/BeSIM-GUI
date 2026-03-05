// Desktop entry point (requires dioxus-desktop feature)
#![allow(non_snake_case)]

use besim_gui_dioxus::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus_desktop::launch(App);
}
