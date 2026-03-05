use gloo_timers::callback::Interval;
use js_sys::Date;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, HtmlInputElement};
use yew::prelude::*;

use crate::api::get_json;
use crate::models::{RoomHistoryPoint, WeatherHistoryPoint};

#[derive(Properties, PartialEq)]
pub struct RoomHistoryProps {
    pub device_id: String,
    pub room_id: String,
}

fn to_iso_date(date: &str) -> Option<String> {
    if date.is_empty() {
        return None;
    }
    let value = format!("{}T00:00:00", date);
    let js_date = Date::new(&JsValue::from_str(&value));
    Some(js_date.to_iso_string().as_string().unwrap_or(value))
}

fn parse_ts(ts: &str) -> Option<f64> {
    let js_date = Date::new(&JsValue::from_str(ts));
    let millis = js_date.get_time();
    if millis.is_nan() {
        None
    } else {
        Some(millis)
    }
}

#[derive(Clone, PartialEq)]
struct SeriesData {
    label: String,
    color: String,
    points: Vec<(f64, f64)>,
}

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
        .x_label_formatter(&|value| format_epoch_label(*value))
        .x_labels(4)
        .y_labels(4)
        .draw();

    for item in series {
        let color = hex_color(&item.color);
        let _ = chart.draw_series(LineSeries::new(item.points.clone(), color.stroke_width(2)));
    }
}

fn format_epoch_label(value: f64) -> String {
    if !value.is_finite() {
        return String::new();
    }
    let date = Date::new(&JsValue::from_f64(value));
    let day = date.get_date();
    let month = date.get_month() + 1;
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    format!("{:02}/{:02} {:02}:{:02}", day, month, hours, minutes)
}

#[function_component(RoomHistoryPage)]
pub fn room_history_page(props: &RoomHistoryProps) -> Html {
    let room_history = use_state(Vec::<RoomHistoryPoint>::new);
    let weather_history = use_state(Vec::<WeatherHistoryPoint>::new);
    let error = use_state(|| None::<String>);
    let canvas_ref = use_node_ref();

    let start_date = use_state(|| {
        let date = Date::new_0();
        let day = date.get_date();
        let adjusted = if day > 2 { day - 2 } else { 1 };
        date.set_date(adjusted);
        date.to_iso_string().as_string().unwrap_or_default()[..10].to_string()
    });
    let end_date = use_state(|| None::<String>);

    {
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        let room_history = room_history.clone();
        let weather_history = weather_history.clone();
        let error = error.clone();
        let start_date = start_date.clone();
        let end_date = end_date.clone();

        use_effect_with(
            (
                device_id,
                room_id,
                (*start_date).clone(),
                (*end_date).clone(),
            ),
            move |(device_id, room_id, start_date, end_date)| {
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                let room_history_state = room_history.clone();
                let weather_history_state = weather_history.clone();
                let error_state = error.clone();

                let start_date = start_date.clone();
                let end_date = end_date.clone();

                let fetch_history = move || {
                    let device_id = device_id.clone();
                    let room_id = room_id.clone();
                    let room_history_state = room_history_state.clone();
                    let weather_history_state = weather_history_state.clone();
                    let error_state = error_state.clone();
                    let start_date = start_date.clone();
                    let end_date = end_date.clone();

                    spawn_local(async move {
                        let from = to_iso_date(&start_date).unwrap_or_default();
                        let to = end_date.as_ref().and_then(|value| to_iso_date(value));
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
                            Ok(data) => room_history_state.set(data),
                            Err(err) => {
                                error_state.set(Some(err));
                                return;
                            }
                        }

                        let weather_url = if let Some(to) = to {
                            format!("weather/history?from={}&to={}", from, to)
                        } else {
                            format!("weather/history?from={}", from)
                        };

                        match get_json::<Vec<WeatherHistoryPoint>>(&weather_url).await {
                            Ok(data) => weather_history_state.set(data),
                            Err(err) => error_state.set(Some(err)),
                        }
                    });
                };

                fetch_history();
                let handle = Interval::new(60000, fetch_history);
                move || drop(handle)
            },
        );
    }

    let on_start_date = {
        let start_date = start_date.clone();
        Callback::from(move |event: Event| {
            if let Some(input) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                start_date.set(input.value());
            }
        })
    };

    let on_end_date = {
        let end_date = end_date.clone();
        Callback::from(move |event: Event| {
            if let Some(input) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                let value = input.value();
                if value.is_empty() {
                    end_date.set(None);
                } else {
                    end_date.set(Some(value));
                }
            }
        })
    };

    if let Some(message) = (*error).clone() {
        return html! { <div class="card error">{ message }</div> };
    }

    let mut temp_points = Vec::new();
    let mut set_points = Vec::new();
    let mut outside_points = Vec::new();
    let mut heating_points = Vec::new();

    for point in room_history.iter() {
        if let Some(ts) = parse_ts(&point.ts) {
            temp_points.push((ts, point.temp));
            set_points.push((ts, point.settemp));
            if let Some(heat) = point.heating {
                heating_points.push((ts, heat));
            }
        }
    }

    for point in weather_history.iter() {
        if let Some(ts) = parse_ts(&point.ts) {
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
        let _max = temp_points
            .iter()
            .chain(set_points.iter())
            .chain(outside_points.iter())
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, |acc, val| acc.max(val));
        heating_points = heating_points
            .into_iter()
            .map(|(ts, heat)| (ts, if min < 0.0 { heat - min } else { heat }))
            .collect();
    }

    let series = vec![
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
    ];

    {
        let canvas_ref = canvas_ref.clone();
        let series = series.clone();
        use_effect_with(series.clone(), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_chart(&canvas, &series);
            }
            || ()
        });
    }

    html! {
        <div class="history">
            <div class="history-chart">
                <canvas ref={canvas_ref} class="history-canvas" width="900" height="600" />
                <div class="chart-legend">
                    { for series.iter().map(|item| html! {
                        <div class="chart-legend-item">
                            <span class="chart-legend-swatch" style={format!("background: {}", item.color)}></span>
                            <span>{item.label.clone()}</span>
                        </div>
                    }) }
                </div>
            </div>
            <div class="history-controls">
                <label>
                    { "Start date" }
                    <input type="date" value={(*start_date).clone()} onchange={on_start_date} />
                </label>
                <label>
                    { "End date" }
                    <input
                        type="date"
                        value={(*end_date).clone().unwrap_or_default()}
                        onchange={on_end_date}
                    />
                </label>
            </div>
        </div>
    }
}
