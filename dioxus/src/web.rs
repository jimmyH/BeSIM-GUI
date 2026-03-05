// Web entry point (wasm)
#![allow(non_snake_case)]

use besim_gui_dioxus::App;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
