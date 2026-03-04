use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use gloo_net::http::Request;

fn from_window_api_url() -> Option<String> {
    let window = web_sys::window()?;
    let value = js_sys::Reflect::get(&window, &JsValue::from_str("__API_URL")).ok()?;
    value.as_string()
}

fn normalize_base(base: String) -> String {
    let trimmed = base.trim().to_string();
    if trimmed.is_empty() {
        return String::new();
    }
    trimmed.trim_end_matches('/').to_string()
}

fn join_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    if base.is_empty() {
        return format!("/{}", path);
    }
    format!("{}/{}", base, path)
}

pub fn api_base_url() -> String {
    let runtime = from_window_api_url().unwrap_or_default();
    let build = option_env!("API_URL").unwrap_or("");
    let base = if !runtime.trim().is_empty() {
        runtime
    } else {
        build.to_string()
    };
    normalize_base(base)
}

pub async fn get_json<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let base = api_base_url();
    let url = join_url(&base, path);
    Request::get(&url)
        .send()
        .await
        .map_err(|err| err.to_string())?
        .json::<T>()
        .await
        .map_err(|err| err.to_string())
}

pub async fn put_json<T: Serialize>(path: &str, body: &T) -> Result<(), String> {
    let base = api_base_url();
    let url = join_url(&base, path);
    Request::put(&url)
        .json(body)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}
