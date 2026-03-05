use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::get_json;
use crate::routes::Route;

#[function_component(DevicesPage)]
pub fn devices_page() -> Html {
    let devices = use_state(Vec::<i32>::new);
    let error = use_state(|| None::<String>);

    {
        let devices = devices.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match get_json::<Vec<i32>>("devices").await {
                    Ok(list) => devices.set(list),
                    Err(err) => error.set(Some(err)),
                }
            });
            || ()
        });
    }

    if let Some(message) = (*error).clone() {
        return html! { <div class="card error">{ message }</div> };
    }

    if devices.is_empty() {
        return html! { <div class="card">{ "Loading devices..." }</div> };
    }

    html! {
        <div class="list">
            { for devices.iter().map(|device| {
                let route = Route::DeviceDetails { id: device.to_string() };
                html! {
                    <Link<Route> to={route} classes="card card-link">
                        <div class="card-title">{device}</div>
                    </Link<Route>>
                }
            }) }
        </div>
    }
}
