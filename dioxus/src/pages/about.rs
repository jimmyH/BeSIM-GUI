use dioxus::prelude::*;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        div { class: "card",
            p { "Test app using Dioxus to consume the BeSMART REST API." }
        }
    }
}
