use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use plotters::prelude::*;

#[cfg(target_arch = "wasm32")]
use plotters_canvas::CanvasBackend;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use crate::api::{self, get_json};
use crate::models::{RoomHistoryPoint, WeatherHistoryPoint};

#[derive(Clone, PartialEq)]
struct SeriesData {
    label: String,
    color: String,
    points: Vec<(f64, f64)>,
}

#[cfg(target_arch = "wasm32")]
fn hex_color(value: &str) -> RGBColor {
    let trimmed = value.trim_start_matches('#');
    if trimmed.len() != 6 {
        return RGBColor(0, 0, 0);
    }
    let r = u8::from_str_radix(&trimmed[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&trimmed[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&trimmed[4..6], 16).unwrap_or(0);
    RGBColor(r, g, b)
}

#[cfg(target_arch = "wasm32")]
fn draw_chart(canvas: &HtmlCanvasElement, series: &[SeriesData]) {
    if series.is_empty() {
        return;
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for item in series {
        for (x, y) in &item.points {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
    }

    if !min_x.is_finite() || !max_x.is_finite() || (max_x - min_x).abs() < f64::EPSILON {
        return;
    }
    if !min_y.is_finite() || !max_y.is_finite() || (max_y - min_y).abs() < f64::EPSILON {
        min_y = 0.0;
        max_y = 1.0;
    }
    if min_y > 0.0 {
        min_y = 0.0;
    }

    let backend = CanvasBackend::with_canvas_object(canvas.clone()).expect("canvas backend");
    let root = backend.into_drawing_area();
    let _ = root.fill(&RGBColor(245, 247, 249));

    let mut chart = ChartBuilder::on(&root)
        .margin(16)
        .x_label_area_size(24)
        .y_label_area_size(36)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .expect("chart build");

    let _ = chart
        .configure_mesh()
        .disable_mesh()
        .x_label_formatter(&|value| api::format_epoch_label(*value))
        .x_labels(4)
        .y_labels(4)
        .draw();

    for item in series {
        let color = hex_color(&item.color);
        let _ = chart.draw_series(LineSeries::new(item.points.clone(), color.stroke_width(2)));
    }
}

fn build_series(
    room_history: &[RoomHistoryPoint],
    weather_history: &[WeatherHistoryPoint],
) -> Vec<SeriesData> {
    let mut temp_points = Vec::new();
    let mut set_points = Vec::new();
    let mut outside_points = Vec::new();
    let mut heating_points = Vec::new();

    for point in room_history {
        if let Some(ts) = api::parse_ts(&point.ts) {
            temp_points.push((ts, point.temp));
            set_points.push((ts, point.settemp));
            if let Some(heat) = point.heating {
                heating_points.push((ts, heat));
            }
        }
    }

    for point in weather_history {
        if let Some(ts) = api::parse_ts(&point.ts) {
            outside_points.push((ts, point.temp));
        }
    }

    if !heating_points.is_empty() {
        let min = temp_points
            .iter()
            .chain(set_points.iter())
            .chain(outside_points.iter())
            .map(|(_, y)| *y)
            .fold(f64::INFINITY, |acc, val| acc.min(val));
        heating_points = heating_points
            .into_iter()
            .map(|(ts, heat)| (ts, if min < 0.0 { heat - min } else { heat }))
            .collect();
    }

    vec![
        SeriesData {
            label: "Temp".to_string(),
            color: "#F25C5C".to_string(),
            points: temp_points,
        },
        SeriesData {
            label: "SetTemp".to_string(),
            color: "#2A92BF".to_string(),
            points: set_points,
        },
        SeriesData {
            label: "Outside".to_string(),
            color: "#8FCB9B".to_string(),
            points: outside_points,
        },
        SeriesData {
            label: "Heating".to_string(),
            color: "#F2C94C".to_string(),
            points: heating_points,
        },
    ]
}

#[component]
pub fn RoomHistoryPage(device_id: String, room_id: String) -> Element {
    let mut room_history = use_signal(Vec::<RoomHistoryPoint>::new);
    let mut weather_history = use_signal(Vec::<WeatherHistoryPoint>::new);
    let mut error = use_signal(|| None::<String>);

    let mut start_date = use_signal(|| api::local_date_minus_days(2));
    let mut end_date = use_signal(|| None::<String>);

    use_resource(move || {
        let device_id = device_id.clone();
        let room_id = room_id.clone();
        let start_date = start_date.read().clone();
        let end_date = end_date.read().clone();

        async move {
            loop {
                let from = api::to_iso_date(&start_date).unwrap_or_default();
                let to = end_date.as_ref().and_then(|value| api::to_iso_date(value));

                let room_url = if let Some(to) = to.clone() {
                    format!(
                        "devices/{}/rooms/{}/history?from={}&to={}",
                        device_id, room_id, from, to
                    )
                } else {
                    format!(
                        "devices/{}/rooms/{}/history?from={}",
                        device_id, room_id, from
                    )
                };

                match get_json::<Vec<RoomHistoryPoint>>(&room_url).await {
                    Ok(data) => {
                        error.set(None);
                        room_history.set(data)
                    }
                    Err(err) => {
                        error.set(Some(err));
                        return;
                    }
                }

                let weather_url = if let Some(to) = to {
                    format!("weather/history?from={}&to={}", from, to)
                } else {
                    format!("weather/history?from={}", from)
                };

                match get_json::<Vec<WeatherHistoryPoint>>(&weather_url).await {
                    Ok(data) => weather_history.set(data),
                    Err(err) => error.set(Some(err)),
                }

                crate::api::sleep(60000).await;
            }
        }
    });

    if let Some(message) = error.read().clone() {
        return rsx! {
            div { class: "card error", "{message}" }
        };
    }

    let room_history_data = room_history.read().clone();
    let weather_history_data = weather_history.read().clone();
    let series = build_series(&room_history_data, &weather_history_data);

    let canvas_id = "history-canvas";

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            let room_history_data = room_history.read().clone();
            let weather_history_data = weather_history.read().clone();
            let series = build_series(&room_history_data, &weather_history_data);

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(canvas) = document.get_element_by_id(canvas_id) {
                    if let Ok(canvas_element) = canvas.dyn_into::<HtmlCanvasElement>() {
                        draw_chart(&canvas_element, &series);
                    }
                }
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    let chart_element = rsx! {
        canvas {
            id: "{canvas_id}",
            class: "history-canvas",
            width: "900",
            height: "600",
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let chart_element = rsx! {
        div { class: "card", "Charts not supported in this mode" }
    };

    rsx! {
        div { class: "history",
            div { class: "history-chart",
                {chart_element}
                div { class: "chart-legend",
                    for item in series.iter() {
                        div { class: "chart-legend-item", key: "{item.label}",
                            span {
                                class: "chart-legend-swatch",
                                style: "background: {item.color}",
                            }
                            span { "{item.label}" }
                        }
                    }
                }
            }
            div { class: "history-controls",
                label {
                    "Start date"
                    input {
                        r#type: "date",
                        value: "{start_date.read()}",
                        onchange: move |event| {
                            start_date.set(event.value());
                        },
                    }
                }
                label {
                    "End date"
                    input {
                        r#type: "date",
                        value: "{end_date.read().clone().unwrap_or_default()}",
                        onchange: move |event| {
                            let value = event.value();
                            if value.is_empty() {
                                end_date.set(None);
                            } else {
                                end_date.set(Some(value));
                            }
                        },
                    }
                }
            }
        }
    }
}
