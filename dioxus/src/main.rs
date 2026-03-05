mod api;
mod app;
mod components;
mod models;
mod pages;
mod routes;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(app::App);
}
