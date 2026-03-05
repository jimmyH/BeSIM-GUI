use dioxus::prelude::*;

#[component]
pub fn NotFoundPage() -> Element {
    rsx! {
        div { class: "card",
            h2 { "Page not found" }
        }
    }
}
