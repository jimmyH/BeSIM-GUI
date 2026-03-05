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

#[derive(Clone, Copy)]
pub struct LocalClock {
    pub weekday: u32,
    pub hours: u32,
    pub minutes: u32,
}

#[cfg(target_arch = "wasm32")]
pub fn local_clock() -> LocalClock {
    let date = js_sys::Date::new_0();
    LocalClock {
        weekday: date.get_day(),
        hours: date.get_hours(),
        minutes: date.get_minutes(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn local_clock() -> LocalClock {
    use chrono::{Datelike, Local, Timelike};

    let now = Local::now();
    LocalClock {
        weekday: now.weekday().num_days_from_sunday(),
        hours: now.hour(),
        minutes: now.minute(),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn local_date_minus_days(days: i64) -> String {
    let date = js_sys::Date::new_0();
    let current = date.get_date() as i64;
    let adjusted = (current - days).max(1) as u32;
    date.set_date(adjusted);
    date.to_iso_string().as_string().unwrap_or_default()[..10].to_string()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn local_date_minus_days(days: i64) -> String {
    use chrono::{Duration, Local};

    let date = Local::now().date_naive() - Duration::days(days);
    date.format("%Y-%m-%d").to_string()
}

#[cfg(target_arch = "wasm32")]
pub fn to_iso_date(date: &str) -> Option<String> {
    use wasm_bindgen::JsValue;

    if date.is_empty() {
        return None;
    }
    let value = format!("{}T00:00:00", date);
    let js_date = js_sys::Date::new(&JsValue::from_str(&value));
    Some(js_date.to_iso_string().as_string().unwrap_or(value))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_iso_date(date: &str) -> Option<String> {
    use chrono::{Local, NaiveDate, TimeZone, Utc};

    if date.is_empty() {
        return None;
    }
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let naive_dt = naive_date.and_hms_opt(0, 0, 0)?;
    let local_dt = Local.from_local_datetime(&naive_dt).single()?;
    Some(local_dt.with_timezone(&Utc).to_rfc3339())
}

#[cfg(target_arch = "wasm32")]
pub fn parse_ts(ts: &str) -> Option<f64> {
    use wasm_bindgen::JsValue;

    let js_date = js_sys::Date::new(&JsValue::from_str(ts));
    let millis = js_date.get_time();
    if millis.is_nan() {
        None
    } else {
        Some(millis)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_ts(ts: &str) -> Option<f64> {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

    if let Ok(dt) = DateTime::parse_from_rfc3339(ts) {
        return Some(dt.timestamp_millis() as f64);
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S") {
        let local_dt = Local.from_local_datetime(&naive).single()?;
        return Some(local_dt.timestamp_millis() as f64);
    }
    None
}

#[cfg(target_arch = "wasm32")]
pub fn format_epoch_label(value: f64) -> String {
    use wasm_bindgen::JsValue;

    if !value.is_finite() {
        return String::new();
    }
    let date = js_sys::Date::new(&JsValue::from_f64(value));
    let day = date.get_date();
    let month = date.get_month() + 1;
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    format!("{:02}/{:02} {:02}:{:02}", day, month, hours, minutes)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn format_epoch_label(value: f64) -> String {
    use chrono::{Datelike, Local, TimeZone, Timelike, Utc};

    if !value.is_finite() {
        return String::new();
    }
    let millis = value.round() as i64;
    let dt = match Utc.timestamp_millis_opt(millis).single() {
        Some(dt) => dt.with_timezone(&Local),
        None => return String::new(),
    };
    format!(
        "{:02}/{:02} {:02}:{:02}",
        dt.day(),
        dt.month(),
        dt.hour(),
        dt.minute()
    )
}

#[cfg(target_arch = "wasm32")]
pub fn format_epoch_seconds(ts: f64) -> String {
    use wasm_bindgen::JsValue;

    let date = js_sys::Date::new(&JsValue::from_f64(ts * 1000.0));
    date.to_string().into()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn format_epoch_seconds(ts: f64) -> String {
    use chrono::{Local, TimeZone};

    let millis = (ts * 1000.0) as i64;
    match Local.timestamp_millis_opt(millis).single() {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => String::new(),
    }
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
