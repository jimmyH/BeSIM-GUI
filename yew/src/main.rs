mod api;
mod app;
mod components;
mod models;
mod pages;
mod routes;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    yew::Renderer::<app::App>::new().render();
}
