use dioxus::prelude::*;
use dioxus_router::components::*;
use js_sys::Date;
use serde_json::Value;
use wasm_bindgen::JsValue;

use crate::api::{get_json, sleep};
use crate::models::DeviceDetails;
use crate::routes::Route;

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

#[component]
pub fn DeviceDetailsPage(id: String) -> Element {
    let mut device = use_signal(|| None::<DeviceDetails>);
    let mut error = use_signal(|| None::<String>);

    let id_clone = id.clone();
    use_resource(move || {
        let id = id_clone.clone();
        async move {
            loop {
                match get_json::<DeviceDetails>(&format!("devices/{}", id)).await {
                    Ok(data) => device.set(Some(data)),
                    Err(err) => error.set(Some(err)),
                }
                sleep(5000).await;
            }
        }
    });

    if let Some(message) = error.read().clone() {
        return rsx! {
            div { class: "card error", "{message}" }
        };
    }

    let Some(device_data) = device.read().clone() else {
        return rsx! {
            div { class: "card", "Loading device..." }
        };
    };

    let addr = device_data
        .addr
        .first()
        .map(format_value)
        .unwrap_or_default();
    let port = device_data
        .addr
        .get(1)
        .map(format_value)
        .unwrap_or_default();

    rsx! {
        div { class: "card stack",
            div {
                span { class: "label", "WiFi Signal: " }
                "{format_value(&device_data.wifi_signal)}"
            }
            div {
                span { class: "label", "Version: " }
                "{format_value(&device_data.version)}"
            }
            div {
                span { class: "label", "IP: " }
                "{addr}:{port}"
            }
            div {
                span { class: "label", "Last Seen: " }
                "{format_last_seen(&device_data.last_seen)}"
            }
            div {
                span { class: "label", "OpenTherm:" }
            }
            div {
                span { class: "label", "Boiler On: " }
                "{format_value(&device_data.boiler_on)}"
            }
            div {
                span { class: "label", "DHW On: " }
                "{format_value(&device_data.dhw_mode)}"
            }
            div {
                span { class: "label", "tFLO (flow sensor temperature): " }
                "{format_value(&device_data.t_flo)}"
            }
            div {
                span { class: "label", "tdH (DHW sensor temperature): " }
                "{format_value(&device_data.t_dh)}"
            }
            div {
                span { class: "label", "tESt (outdoor sensor temperature): " }
                "{format_value(&device_data.t_est)}"
            }
            Link {
                to: Route::Rooms {
                    device_id: id.clone(),
                },
                class: "button",
                "Rooms"
            }
        }
    }
}
