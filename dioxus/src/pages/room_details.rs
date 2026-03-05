use dioxus::prelude::*;
use dioxus_router::components::*;
use gloo_timers::future::TimeoutFuture;
use js_sys::Date;
use serde_json::Value;

use crate::api::{get_json, put_json};
use crate::components::schedule_chart::ScheduleChart;
use crate::models::Room;
use crate::routes::Route;

#[derive(Clone, PartialEq)]
struct DayOption {
    value: u8,
    label: &'static str,
}

fn day_options() -> Vec<DayOption> {
    vec![
        DayOption {
            value: 0,
            label: "Sunday",
        },
        DayOption {
            value: 1,
            label: "Monday",
        },
        DayOption {
            value: 2,
            label: "Tuesday",
        },
        DayOption {
            value: 3,
            label: "Wednesday",
        },
        DayOption {
            value: 4,
            label: "Thursday",
        },
        DayOption {
            value: 5,
            label: "Friday",
        },
        DayOption {
            value: 6,
            label: "Saturday",
        },
    ]
}

fn current_day() -> u8 {
    Date::new_0().get_day() as u8
}

fn weather_temp(weather: &Value) -> Option<f64> {
    let temp = weather
        .get("properties")?
        .get("timeseries")?
        .get(0)?
        .get("data")?
        .get("instant")?
        .get("details")?
        .get("air_temperature")?
        .as_f64()?;
    Some(temp)
}

fn weather_symbol(weather: &Value) -> Option<String> {
    weather
        .get("properties")?
        .get("timeseries")?
        .get(0)?
        .get("data")?
        .get("next_6_hours")?
        .get("summary")?
        .get("symbol_code")?
        .as_str()
        .map(|value| value.to_string())
}

fn display_temp(room: &Room, temp: i32) -> String {
    let mut value = temp as f64 / 10.0;
    if room.units != 0 {
        value = value * 9.0 / 5.0 + 32.0;
    }
    format!("{:.1}", value)
}

fn ensure_days(days: &mut Vec<Vec<u8>>) {
    if days.len() < 7 {
        days.resize_with(7, || vec![0; 24]);
    }
    for day in days.iter_mut() {
        if day.len() < 24 {
            day.resize(24, 0);
        }
    }
}

fn get_prog(room: &Room, dow: u8, idx: usize) -> u8 {
    let day_idx = idx / 2;
    let nibble = idx % 2;
    let value = room
        .days
        .get(dow as usize)
        .and_then(|day| day.get(day_idx))
        .copied()
        .unwrap_or(0);

    if nibble == 0 {
        value & 0x0f
    } else {
        (value >> 4) & 0x0f
    }
}

fn update_prog(days: &mut Vec<Vec<u8>>, dow: u8, idx: usize, new_value: u8) {
    ensure_days(days);
    let day_idx = idx / 2;
    let nibble = idx % 2;
    if let Some(day) = days.get_mut(dow as usize) {
        let old = day.get(day_idx).copied().unwrap_or(0);
        let updated = if nibble == 0 {
            (old & 0xf0) | (new_value & 0x0f)
        } else {
            (old & 0x0f) | ((new_value & 0x0f) << 4)
        };
        if day.len() <= day_idx {
            day.resize(day_idx + 1, 0);
        }
        day[day_idx] = updated;
    }
}

fn schedule_data(room: &Room, day: u8) -> Vec<u8> {
    let mut data = Vec::with_capacity(48);
    for idx in 0..48 {
        let value = get_prog(room, day, idx) + 1;
        data.push(value);
    }
    data
}

fn update_room_value<F>(
    room: &mut Signal<Option<Room>>,
    device_id: &str,
    room_id: &str,
    endpoint: &str,
    updater: F,
) where
    F: FnOnce(&mut Room) -> i32,
{
    let device_id = device_id.to_string();
    let room_id = room_id.to_string();
    let endpoint = endpoint.to_string();

    let (room_data, val) = {
        let current = room.read().clone();
        if let Some(mut room_data) = current {
            let val = updater(&mut room_data);
            (Some(room_data), val)
        } else {
            return;
        }
    };

    room.set(room_data);
    spawn(async move {
        let _ = put_json(
            &format!("devices/{}/rooms/{}/{}", device_id, room_id, endpoint),
            &val,
        )
        .await;
    });
}

#[component]
pub fn RoomDetailsPage(device_id: String, room_id: String) -> Element {
    let mut room = use_signal(|| None::<Room>);
    let mut weather = use_signal(|| None::<Value>);
    let mut error = use_signal(|| None::<String>);
    let mut selected_day = use_signal(current_day);

    let device_id_clone = device_id.clone();
    let room_id_clone = room_id.clone();
    use_resource(move || {
        let device_id = device_id_clone.clone();
        let room_id = room_id_clone.clone();
        async move {
            loop {
                match get_json::<Room>(&format!("devices/{}/rooms/{}", device_id, room_id)).await {
                    Ok(data) => room.set(Some(data)),
                    Err(err) => error.set(Some(err)),
                }
                TimeoutFuture::new(5000).await;
            }
        }
    });

    use_resource(move || async move {
        loop {
            match get_json::<Value>("weather").await {
                Ok(data) => weather.set(Some(data)),
                Err(err) => error.set(Some(err)),
            }
            TimeoutFuture::new(600000).await;
        }
    });

    if let Some(message) = error.read().clone() {
        return rsx! {
            div { class: "card error", "{message}" }
        };
    }

    let Some(room_data) = room.read().clone() else {
        return rsx! {
            div { class: "card", "Loading room..." }
        };
    };

    let weather_temp_value = weather.read().as_ref().and_then(weather_temp);
    let weather_symbol_value = weather.read().as_ref().and_then(weather_symbol);

    let schedule = schedule_data(&room_data, *selected_day.read());

    // Clone for schedule selector
    let device_id_sched = device_id.clone();
    let room_id_sched = room_id.clone();

    rsx! {
        div { class: "container",
            div { class: "main",
                h1 { "{display_temp(&room_data, room_data.temp)}°" }
                div { class: "weather",
                    {weather_temp_value.map(|temp| rsx! {
                        span { "{temp:.1}°" }
                    })}
                    {weather_symbol_value.clone().map(|code| rsx! {
                        img { src: "/assets/weather/{code}.png", alt: "{code}" }
                    })}
                }
                div { class: "field",
                    label { r#for: "day-select", "Day" }
                    select {
                        id: "day-select",
                        onchange: move |event| {
                            if let Ok(value) = event.value().parse::<u8>() {
                                selected_day.set(value);
                            }
                        },
                        for day in day_options() {
                            option {
                                value: "{day.value}",
                                selected: *selected_day.read() == day.value,
                                "{day.label}"
                            }
                        }
                    }
                }
            }
            div { class: "graph",
                ScheduleChart {
                    data: schedule.clone(),
                    on_select: move |idx: usize| {
                        let Some(mut room_data) = room.read().clone() else {
                            return;
                        };
                        let mut days = room_data.days.clone();
                        let day = *selected_day.read();
                        let current = get_prog(&room_data, day, idx);
                        let next = (current + 1) % 3;
                        update_prog(&mut days, day, idx, next);
                        room_data.days = days.clone();
                        room.set(Some(room_data));

                        let device_id = device_id_sched.clone();
                        let room_id = room_id_sched.clone();
                        spawn(async move {
                            let _ = put_json(
                                    &format!("devices/{}/rooms/{}/days/{}", device_id, room_id, day),
                                    &days[day as usize],
                                )
                                .await;
                        });
                    },
                    selected_day: *selected_day.read(),
                }
            }
            div { class: "nav",
                if room_data.lowbattery == 1 {
                    {"🪫"}
                } else {
                    {"🔋"}
                }
                if room_data.cmdissued == 1 {
                    "↔"
                }
                if room_data.heating {
                    "🔥"
                }
                if room_data.winter == 0 {
                    div { class: "badge", "Cool" }
                }
                if room_data.advance == 1 {
                    div { class: "badge", "A" }
                }
                if room_data.boost == 1 || room_data.fakeboost != 0 {
                    div { class: "badge", "B" }
                }
            }
            div { class: "aside",
                {
                    let device_id_t1_up = device_id.clone();
                    let room_id_t1_up = room_id.clone();
                    let device_id_t1_down = device_id.clone();
                    let room_id_t1_down = room_id.clone();
                    rsx! {
                        div { class: if room_data.t1 == room_data.settemp { "temp-bold" } else { "temp-norm" },
                            div { "T1: {display_temp(&room_data, room_data.t1)}°" }
                            button {
                                class: "icon-button",
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_t1_up,
                                        &room_id_t1_up,
                                        "t1",
                                        |r| {
                                            r.t1 += 2;
                                            r.t1
                                        },
                                    );
                                },
                                "▲"
                            }
                            button {
                                class: "icon-button",
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_t1_down,
                                        &room_id_t1_down,
                                        "t1",
                                        |r| {
                                            r.t1 -= 2;
                                            r.t1
                                        },
                                    );
                                },
                                "▼"
                            }
                        }
                    }
                }
                {
                    let device_id_t2_up = device_id.clone();
                    let room_id_t2_up = room_id.clone();
                    let device_id_t2_down = device_id.clone();
                    let room_id_t2_down = room_id.clone();
                    rsx! {
                        div { class: if room_data.t2 == room_data.settemp { "temp-bold" } else { "temp-norm" },
                            div { "T2: {display_temp(&room_data, room_data.t2)}°" }
                            button {
                                class: "icon-button",
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_t2_up,
                                        &room_id_t2_up,
                                        "t2",
                                        |r| {
                                            r.t2 += 2;
                                            r.t2
                                        },
                                    );
                                },
                                "▲"
                            }
                            button {
                                class: "icon-button",
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_t2_down,
                                        &room_id_t2_down,
                                        "t2",
                                        |r| {
                                            r.t2 -= 2;
                                            r.t2
                                        },
                                    );
                                },
                                "▼"
                            }
                        }
                    }
                }
                {
                    let device_id_t3_up = device_id.clone();
                    let room_id_t3_up = room_id.clone();
                    let device_id_t3_down = device_id.clone();
                    let room_id_t3_down = room_id.clone();
                    rsx! {
                        div { class: if room_data.t3 == room_data.settemp { "temp-bold" } else { "temp-norm" },
                            div { "T3: {display_temp(&room_data, room_data.t3)}°" }
                            button {
                                class: "icon-button",
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_t3_up,
                                        &room_id_t3_up,
                                        "t3",
                                        |r| {
                                            r.t3 += 2;
                                            r.t3
                                        },
                                    );
                                },
                                "▲"
                            }
                            button {
                                class: "icon-button",
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_t3_down,
                                        &room_id_t3_down,
                                        "t3",
                                        |r| {
                                            r.t3 -= 2;
                                            r.t3
                                        },
                                    );
                                },
                                "▼"
                            }
                        }
                    }
                }
            }
            div { class: "footer",
                {
                    let device_id_season_cool = device_id.clone();
                    let room_id_season_cool = room_id.clone();
                    let device_id_season_heat = device_id.clone();
                    let room_id_season_heat = room_id.clone();
                    rsx! {
                        div { class: "toggle-group",
                            span { "Season" }
                            button {
                                class: if room_data.winter == 0 { "toggle active" } else { "toggle" },
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_season_cool,
                                        &room_id_season_cool,
                                        "winter",
                                        |r| {
                                            r.winter = 0;
                                            0
                                        },
                                    );
                                },
                                "Cooling"
                            }
                            button {
                                class: if room_data.winter == 1 { "toggle active" } else { "toggle" },
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_season_heat,
                                        &room_id_season_heat,
                                        "winter",
                                        |r| {
                                            r.winter = 1;
                                            1
                                        },
                                    );
                                },
                                "Heating"
                            }
                        }
                    }
                }
                div { class: "toggle-group",
                    span { "Mode" }
                    for (idx , label) in ["Auto", "Manual", "Holiday", "Party", "Off", "DHW"].iter().enumerate() {
                        {
                            let device_id_mode = device_id.clone();
                            let room_id_mode = room_id.clone();
                            let mode_idx = idx;
                            rsx! {
                                button {
                                    key: "{idx}",
                                    class: if room_data.mode == idx as i32 { "toggle active" } else { "toggle" },
                                    onclick: move |_| {
                                        update_room_value(
                                            &mut room,
                                            &device_id_mode,
                                            &room_id_mode,
                                            "mode",
                                            |r| {
                                                r.mode = mode_idx as i32;
                                                mode_idx as i32
                                            },
                                        );
                                    },
                                    "{label}"
                                }
                            }
                        }
                    }
                }
                {
                    let device_id_units_c = device_id.clone();
                    let room_id_units_c = room_id.clone();
                    let device_id_units_f = device_id.clone();
                    let room_id_units_f = room_id.clone();
                    rsx! {
                        div { class: "toggle-group",
                            span { "Units" }
                            button {
                                class: if room_data.units == 0 { "toggle active" } else { "toggle" },
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_units_c,
                                        &room_id_units_c,
                                        "units",
                                        |r| {
                                            r.units = 0;
                                            0
                                        },
                                    );
                                },
                                "°C"
                            }
                            button {
                                class: if room_data.units == 1 { "toggle active" } else { "toggle" },
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_units_f,
                                        &room_id_units_f,
                                        "units",
                                        |r| {
                                            r.units = 1;
                                            1
                                        },
                                    );
                                },
                                "°F"
                            }
                        }
                    }
                }
                {
                    let device_id_boost = device_id.clone();
                    let room_id_boost = room_id.clone();
                    let device_id_advance = device_id.clone();
                    let room_id_advance = room_id.clone();
                    let device_id_graphs = device_id.clone();
                    let room_id_graphs = room_id.clone();
                    rsx! {
                        div { class: "footer-actions",
                            button {
                                class: if room_data.boost == 1 || room_data.fakeboost != 0 { "toggle active" } else { "toggle" },
                                onclick: move |_| {
                                    let current = room.read().clone();
                                    if let Some(mut room_data) = current {
                                        if room_data.boost == 0 {
                                            room_data.fakeboost = if room_data.fakeboost == 0 { 1 } else { 0 };
                                            let val = room_data.fakeboost;
                                            let device_id = device_id_boost.clone();
                                            let room_id = room_id_boost.clone();
                                            room.set(Some(room_data));
                                            spawn(async move {
                                                let _ = put_json(
                                                        &format!(
                                                            "devices/{}/rooms/{}/fakeboost",
                                                            device_id,
                                                            room_id,
                                                        ),
                                                        &val,
                                                    )
                                                    .await;
                                            });
                                        }
                                    }
                                },
                                "Boost"
                            }
                            button {
                                class: if room_data.advance == 1 { "toggle active" } else { "toggle" },
                                onclick: move |_| {
                                    update_room_value(
                                        &mut room,
                                        &device_id_advance,
                                        &room_id_advance,
                                        "advance",
                                        |r| {
                                            r.advance = (r.advance + 1) % 2;
                                            r.advance
                                        },
                                    );
                                },
                                "Advance"
                            }
                            Link {
                                to: Route::RoomHistory {
                                    device_id: device_id_graphs.clone(),
                                    room_id: room_id_graphs.clone(),
                                },
                                class: "button",
                                "Graphs"
                            }
                        }
                    }
                }
            }
        }
    }
}
