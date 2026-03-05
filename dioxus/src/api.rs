use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(target_arch = "wasm32")]
fn from_window_api_url() -> Option<String> {
    use wasm_bindgen::JsValue;
    let window = web_sys::window()?;
    let value = js_sys::Reflect::get(&window, &JsValue::from_str("__API_URL")).ok()?;
    value.as_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn from_window_api_url() -> Option<String> {
    // Desktop: use environment variable or None
    std::env::var("API_URL").ok()
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

// Cross-platform sleep helper
#[cfg(target_arch = "wasm32")]
pub async fn sleep(ms: u32) {
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(ms).await;
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep(ms: u32) {
    tokio::time::sleep(std::time::Duration::from_millis(ms as u64)).await;
}

pub async fn get_json<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let base = api_base_url();
    let url = join_url(&base, path);
    reqwest::get(&url)
        .await
        .map_err(|err| err.to_string())?
        .json::<T>()
        .await
        .map_err(|err| err.to_string())
}

pub async fn put_json<T: Serialize>(path: &str, body: &T) -> Result<(), String> {
    let base = api_base_url();
    let url = join_url(&base, path);
    let client = reqwest::Client::new();
    client
        .put(&url)
        .json(body)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}
