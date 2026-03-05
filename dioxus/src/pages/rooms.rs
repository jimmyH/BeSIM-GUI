use dioxus::prelude::*;

use crate::api::get_json;
use crate::routes::Route;

#[component]
pub fn RoomsPage(device_id: String) -> Element {
    let mut rooms = use_signal(Vec::<i32>::new);
    let mut error = use_signal(|| None::<String>);

    let device_id_clone = device_id.clone();
    use_resource(move || {
        let device_id = device_id_clone.clone();
        async move {
            match get_json::<Vec<i32>>(&format!("devices/{}/rooms", device_id)).await {
                Ok(list) => rooms.set(list),
                Err(err) => error.set(Some(err)),
            }
        }
    });

    if let Some(message) = error.read().clone() {
        return rsx! {
            div { class: "card error", "{message}" }
        };
    }

    if rooms.read().is_empty() {
        return rsx! {
            div { class: "card", "Loading rooms..." }
        };
    }

    rsx! {
        div { class: "list",
            for room in rooms.read().iter() {
                Link {
                    to: Route::RoomDetails {
                        device_id: device_id.clone(),
                        room_id: room.to_string(),
                    },
                    class: "card card-link",
                    div { class: "card-title", "{room}" }
                }
            }
        }
    }
}
