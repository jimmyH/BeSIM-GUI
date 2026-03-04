use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::get_json;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct RoomsProps {
    pub device_id: String,
}

#[function_component(RoomsPage)]
pub fn rooms_page(props: &RoomsProps) -> Html {
    let rooms = use_state(Vec::<i32>::new);
    let error = use_state(|| None::<String>);

    {
        let device_id = props.device_id.clone();
        let rooms = rooms.clone();
        let error = error.clone();
        use_effect_with(device_id, move |device_id| {
            let device_id = device_id.clone();
            let rooms = rooms.clone();
            let error = error.clone();
            spawn_local(async move {
                match get_json::<Vec<i32>>(&format!("devices/{}/rooms", device_id)).await {
                    Ok(list) => rooms.set(list),
                    Err(err) => error.set(Some(err)),
                }
            });
            || ()
        });
    }

    if let Some(message) = (*error).clone() {
        return html! { <div class="card error">{message}</div> };
    }

    if rooms.is_empty() {
        return html! { <div class="card">{"Loading rooms..."}</div> };
    }

    html! {
        <div class="list">
            { for rooms.iter().map(|room| {
                let route = Route::RoomDetails {
                    device_id: props.device_id.clone(),
                    room_id: room.to_string(),
                };
                html! {
                    <Link<Route> to={route} classes="card card-link">
                        <div class="card-title">{room}</div>
                    </Link<Route>>
                }
            }) }
        </div>
    }
}
