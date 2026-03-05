use dioxus::prelude::*;

use crate::api::get_json;
use crate::routes::Route;

#[component]
pub fn DevicesPage() -> Element {
    let mut devices = use_signal(Vec::<i32>::new);
    let mut error = use_signal(|| None::<String>);

    use_resource(move || async move {
        match get_json::<Vec<i32>>("devices").await {
            Ok(list) => devices.set(list),
            Err(err) => error.set(Some(err)),
        }
    });

    if let Some(message) = error.read().clone() {
        return rsx! {
            div { class: "card error", "{message}" }
        };
    }

    if devices.read().is_empty() {
        return rsx! {
            div { class: "card", "Loading devices..." }
        };
    }

    rsx! {
        div { class: "list",
            for device in devices.read().iter() {
                Link {
                    to: Route::DeviceDetails {
                        id: device.to_string(),
                    },
                    class: "card card-link",
                    div { class: "card-title", "{device}" }
                }
            }
        }
    }
}
