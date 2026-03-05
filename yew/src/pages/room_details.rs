use gloo_timers::callback::Interval;
use js_sys::Date;
use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

use crate::api::{get_json, put_json};
use crate::components::schedule_chart::ScheduleChart;
use crate::models::Room;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct RoomDetailsProps {
    pub device_id: String,
    pub room_id: String,
}

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

#[function_component(RoomDetailsPage)]
pub fn room_details_page(props: &RoomDetailsProps) -> Html {
    let room = use_state(|| None::<Room>);
    let weather = use_state(|| None::<Value>);
    let error = use_state(|| None::<String>);
    let selected_day = use_state(current_day);

    {
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        let room = room.clone();
        let weather = weather.clone();
        let error = error.clone();

        use_effect_with((device_id, room_id), move |(device_id, room_id)| {
            let device_id_room = device_id.clone();
            let room_id_room = room_id.clone();
            let room_state = room.clone();
            let error_state = error.clone();

            let fetch_room = move || {
                let device_id = device_id_room.clone();
                let room_id = room_id_room.clone();
                let room_state = room_state.clone();
                let error_state = error_state.clone();
                spawn_local(async move {
                    match get_json::<Room>(&format!("devices/{}/rooms/{}", device_id, room_id))
                        .await
                    {
                        Ok(data) => room_state.set(Some(data)),
                        Err(err) => error_state.set(Some(err)),
                    }
                });
            };

            let weather_state = weather.clone();
            let error_state_weather = error.clone();
            let fetch_weather = move || {
                let weather_state = weather_state.clone();
                let error_state = error_state_weather.clone();
                spawn_local(async move {
                    match get_json::<Value>("weather").await {
                        Ok(data) => weather_state.set(Some(data)),
                        Err(err) => error_state.set(Some(err)),
                    }
                });
            };

            fetch_room();
            fetch_weather();
            let room_interval = Interval::new(5000, fetch_room);
            let weather_interval = Interval::new(600000, fetch_weather);
            move || {
                drop(room_interval);
                drop(weather_interval);
            }
        });
    }

    if let Some(message) = (*error).clone() {
        return html! { <div class="card error">{ message }</div> };
    }

    let Some(room_data) = (*room).clone() else {
        return html! { <div class="card">{ "Loading room..." }</div> };
    };

    let weather_temp_value = (*weather).as_ref().and_then(weather_temp);
    let weather_symbol_value = (*weather).as_ref().and_then(weather_symbol);

    let schedule = schedule_data(&room_data, *selected_day);

    let on_day_change = {
        let selected_day = selected_day.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(select) = target {
                if let Ok(value) = select.value().parse::<u8>() {
                    selected_day.set(value);
                }
            }
        })
    };

    let on_schedule_select = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        let selected_day = selected_day.clone();
        Callback::from(move |idx: usize| {
            let Some(mut room_data) = (*room).clone() else {
                return;
            };
            let mut days = room_data.days.clone();
            let day = *selected_day;
            let current = get_prog(&room_data, day, idx);
            let next = (current + 1) % 3;
            update_prog(&mut days, day, idx, next);
            room_data.days = days.clone();
            room.set(Some(room_data));

            let device_id = device_id.clone();
            let room_id = room_id.clone();
            spawn_local(async move {
                let _ = put_json(
                    &format!("devices/{}/rooms/{}/days/{}", device_id, room_id, day),
                    &days[day as usize],
                )
                .await;
            });
        })
    };

    let on_units_change = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |val: i32| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.units = val;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(
                        &format!("devices/{}/rooms/{}/units", device_id, room_id),
                        &val,
                    )
                    .await;
                });
            }
        })
    };

    let on_mode_change = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |val: i32| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.mode = val;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(
                        &format!("devices/{}/rooms/{}/mode", device_id, room_id),
                        &val,
                    )
                    .await;
                });
            }
        })
    };

    let on_season_change = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |val: i32| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.winter = val;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(
                        &format!("devices/{}/rooms/{}/winter", device_id, room_id),
                        &val,
                    )
                    .await;
                });
            }
        })
    };

    let on_advance_toggle = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |_| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.advance = (room_data.advance + 1) % 2;
                let val = room_data.advance;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(
                        &format!("devices/{}/rooms/{}/advance", device_id, room_id),
                        &val,
                    )
                    .await;
                });
            }
        })
    };

    let on_boost_toggle = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |_| {
            if let Some(mut room_data) = (*room).clone() {
                if room_data.boost == 0 {
                    room_data.fakeboost = if room_data.fakeboost == 0 { 1 } else { 0 };
                    let val = room_data.fakeboost;
                    room.set(Some(room_data));
                    let device_id = device_id.clone();
                    let room_id = room_id.clone();
                    spawn_local(async move {
                        let _ = put_json(
                            &format!("devices/{}/rooms/{}/fakeboost", device_id, room_id),
                            &val,
                        )
                        .await;
                    });
                }
            }
        })
    };

    let on_t1 = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |delta: i32| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.t1 += delta;
                let val = room_data.t1;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(&format!("devices/{}/rooms/{}/t1", device_id, room_id), &val)
                        .await;
                });
            }
        })
    };

    let on_t2 = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |delta: i32| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.t2 += delta;
                let val = room_data.t2;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(&format!("devices/{}/rooms/{}/t2", device_id, room_id), &val)
                        .await;
                });
            }
        })
    };

    let on_t3 = {
        let room = room.clone();
        let device_id = props.device_id.clone();
        let room_id = props.room_id.clone();
        Callback::from(move |delta: i32| {
            if let Some(mut room_data) = (*room).clone() {
                room_data.t3 += delta;
                let val = room_data.t3;
                room.set(Some(room_data));
                let device_id = device_id.clone();
                let room_id = room_id.clone();
                spawn_local(async move {
                    let _ = put_json(&format!("devices/{}/rooms/{}/t3", device_id, room_id), &val)
                        .await;
                });
            }
        })
    };

    html! {
        <div class="container">
            <div class="main">
                <h1>{ format!("{}°", display_temp(&room_data, room_data.temp)) }</h1>
                <div class="weather">
                    { weather_temp_value.map(|temp| html! { <span>{format!("{:.1}°", temp)}</span> }).unwrap_or_default() }
                    { weather_symbol_value.clone().map(|code| html! {
                        <img src={format!("/assets/weather/{}.png", code)} alt={code} />
                    }).unwrap_or_default() }
                </div>
                <div class="field">
                    <label for="day-select">{ "Day" }</label>
                    <select id="day-select" onchange={on_day_change}>
                        { for day_options().iter().map(|day| {
                            let is_selected = *selected_day == day.value;
                            html! {
                                <option value={day.value.to_string()} selected={is_selected}>{day.label}</option>
                            }
                        }) }
                    </select>
                </div>
            </div>
            <div class="graph">
                <ScheduleChart
                    data={schedule}
                    on_select={on_schedule_select}
                    selected_day={*selected_day}
                />
            </div>
            <div class="nav">
                { if room_data.lowbattery == 1 {
                    html! { <Icon data={IconData::BOOTSTRAP_BATTERY} width={"24px"} height={"24px"} /> }
                } else {
                    html! { <Icon data={IconData::BOOTSTRAP_BATTERY_FULL} width={"24px"} height={"24px"} /> }
                } }
                { if room_data.cmdissued == 1 { html! { <Icon data={IconData::BOOTSTRAP_ARROW_LEFT_RIGHT} width={"24px"} height={"24px"} /> } } else { html! {} } }
                { if room_data.heating { html! { <Icon data={IconData::BOOTSTRAP_FIRE} width={"24px"} height={"24px"} /> } } else { html! {} } }
                { if room_data.winter == 0 { html! { <div class="badge">{"Cool"}</div> } } else { html! {} } }
                { if room_data.advance == 1 { html! { <div class="badge">{"A"}</div> } } else { html! {} } }
                { if room_data.boost == 1 || room_data.fakeboost != 0 { html! { <div class="badge">{"B"}</div> } } else { html! {} } }
            </div>
            <div class="aside">
                <div
                    class={if room_data.t1 == room_data.settemp { "temp-bold" } else { "temp-norm" }}
                >
                    <div>{ format!("T1: {}°", display_temp(&room_data, room_data.t1)) }</div>
                    <button
                        class="icon-button"
                        onclick={{
                        let on_t1 = on_t1.clone();
                        Callback::from(move |_| on_t1.emit(2))
                    }}
                    >
                        { "▲" }
                    </button>
                    <button
                        class="icon-button"
                        onclick={{
                        let on_t1 = on_t1.clone();
                        Callback::from(move |_| on_t1.emit(-2))
                    }}
                    >
                        { "▼" }
                    </button>
                </div>
                <div
                    class={if room_data.t2 == room_data.settemp { "temp-bold" } else { "temp-norm" }}
                >
                    <div>{ format!("T2: {}°", display_temp(&room_data, room_data.t2)) }</div>
                    <button
                        class="icon-button"
                        onclick={{
                        let on_t2 = on_t2.clone();
                        Callback::from(move |_| on_t2.emit(2))
                    }}
                    >
                        { "▲" }
                    </button>
                    <button
                        class="icon-button"
                        onclick={{
                        let on_t2 = on_t2.clone();
                        Callback::from(move |_| on_t2.emit(-2))
                    }}
                    >
                        { "▼" }
                    </button>
                </div>
                <div
                    class={if room_data.t3 == room_data.settemp { "temp-bold" } else { "temp-norm" }}
                >
                    <div>{ format!("T3: {}°", display_temp(&room_data, room_data.t3)) }</div>
                    <button
                        class="icon-button"
                        onclick={{
                        let on_t3 = on_t3.clone();
                        Callback::from(move |_| on_t3.emit(2))
                    }}
                    >
                        { "▲" }
                    </button>
                    <button
                        class="icon-button"
                        onclick={{
                        let on_t3 = on_t3.clone();
                        Callback::from(move |_| on_t3.emit(-2))
                    }}
                    >
                        { "▼" }
                    </button>
                </div>
            </div>
            <div class="footer">
                <div class="toggle-group">
                    <span>{ "Season" }</span>
                    <button
                        class={if room_data.winter == 0 { "toggle active" } else { "toggle" }}
                        onclick={{
                        let on_season_change = on_season_change.clone();
                        Callback::from(move |_| on_season_change.emit(0))
                    }}
                    >
                        { "Cooling" }
                    </button>
                    <button
                        class={if room_data.winter == 1 { "toggle active" } else { "toggle" }}
                        onclick={{
                        let on_season_change = on_season_change.clone();
                        Callback::from(move |_| on_season_change.emit(1))
                    }}
                    >
                        { "Heating" }
                    </button>
                </div>
                <div class="toggle-group">
                    <span>{ "Mode" }</span>
                    { for ["Auto", "Manual", "Holiday", "Party", "Off", "DHW"].iter().enumerate().map(|(idx, label)| {
                        let active = room_data.mode == idx as i32;
                        let on_mode_change = on_mode_change.clone();
                        html! {
                            <button class={if active { "toggle active" } else { "toggle" }} onclick={Callback::from(move |_| on_mode_change.emit(idx as i32))}>{*label}</button>
                        }
                    }) }
                </div>
                <div class="toggle-group">
                    <span>{ "Units" }</span>
                    <button
                        class={if room_data.units == 0 { "toggle active" } else { "toggle" }}
                        onclick={{
                        let on_units_change = on_units_change.clone();
                        Callback::from(move |_| on_units_change.emit(0))
                    }}
                    >
                        { "°C" }
                    </button>
                    <button
                        class={if room_data.units == 1 { "toggle active" } else { "toggle" }}
                        onclick={{
                        let on_units_change = on_units_change.clone();
                        Callback::from(move |_| on_units_change.emit(1))
                    }}
                    >
                        { "°F" }
                    </button>
                </div>
                <div class="footer-actions">
                    <button
                        class={if room_data.boost == 1 || room_data.fakeboost != 0 { "toggle active" } else { "toggle" }}
                        onclick={on_boost_toggle}
                    >
                        { "Boost" }
                    </button>
                    <button
                        class={if room_data.advance == 1 { "toggle active" } else { "toggle" }}
                        onclick={on_advance_toggle}
                    >
                        { "Advance" }
                    </button>
                    <Link<Route>
                        classes="button"
                        to={Route::RoomHistory { device_id: props.device_id.clone(), room_id: props.room_id.clone() }}
                    >
                        { "Graphs" }
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
