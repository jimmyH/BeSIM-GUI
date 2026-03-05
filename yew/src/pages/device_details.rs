use gloo_timers::callback::Interval;
use js_sys::Date;
use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::get_json;
use crate::models::DeviceDetails;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct DeviceDetailsProps {
    pub id: String,
}

fn format_value(value: &Value) -> String {
    if value.is_null() {
        return "".to_string();
    }
    if let Some(text) = value.as_str() {
        return text.to_string();
    }
    value.to_string()
}

fn format_last_seen(value: &Value) -> String {
    if let Some(ts) = value.as_f64() {
        let date = Date::new(&JsValue::from_f64(ts * 1000.0));
        return date.to_string().into();
    }
    format_value(value)
}

#[function_component(DeviceDetailsPage)]
pub fn device_details_page(props: &DeviceDetailsProps) -> Html {
    let device = use_state(|| None::<DeviceDetails>);
    let error = use_state(|| None::<String>);

    {
        let id = props.id.clone();
        let device = device.clone();
        let error = error.clone();
        use_effect_with(id.clone(), move |_| {
            let id_refresh = id.clone();
            let device_refresh = device.clone();
            let error_refresh = error.clone();

            let refresh = move || {
                let id = id_refresh.clone();
                let device = device_refresh.clone();
                let error = error_refresh.clone();
                spawn_local(async move {
                    match get_json::<DeviceDetails>(&format!("devices/{}", id)).await {
                        Ok(data) => device.set(Some(data)),
                        Err(err) => error.set(Some(err)),
                    }
                });
            };

            refresh();
            let handle = Interval::new(5000, refresh);
            move || drop(handle)
        });
    }

    if let Some(message) = (*error).clone() {
        return html! { <div class="card error">{message}</div> };
    }

    let Some(device) = (*device).clone() else {
        return html! { <div class="card">{"Loading device..."}</div> };
    };

    let addr = device.addr.first().map(format_value).unwrap_or_default();
    let port = device.addr.get(1).map(format_value).unwrap_or_default();

    html! {
        <div class="card stack">
            <div><span class="label">{"WiFi Signal: "}</span>{format_value(&device.wifi_signal)}</div>
            <div><span class="label">{"Version: "}</span>{format_value(&device.version)}</div>
            <div><span class="label">{"IP: "}</span>{format!("{}:{}", addr, port)}</div>
            <div><span class="label">{"Last Seen: "}</span>{format_last_seen(&device.last_seen)}</div>
            <div><span class="label">{"OpenTherm:"}</span></div>
            <div><span class="label">{"Boiler On: "}</span>{format_value(&device.boiler_on)}</div>
            <div><span class="label">{"DHW On: "}</span>{format_value(&device.dhw_mode)}</div>
            <div><span class="label">{"tFLO (flow sensor temperature): "}</span>{format_value(&device.t_flo)}</div>
            <div><span class="label">{"tdH (DHW sensor temperature): "}</span>{format_value(&device.t_dh)}</div>
            <div><span class="label">{"tESt (outdoor sensor temperature): "}</span>{format_value(&device.t_est)}</div>
            <Link<Route> classes="button" to={Route::Rooms { device_id: props.id.clone() }}>
                {"Rooms"}
            </Link<Route>>
        </div>
    }
}
