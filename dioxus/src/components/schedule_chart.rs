use dioxus::prelude::*;
use js_sys::Date;

#[component]
pub fn ScheduleChart(data: Vec<u8>, on_select: EventHandler<usize>, selected_day: u8) -> Element {
    let mut hovered = use_signal(|| None::<usize>);

    let today_idx = {
        let now = Date::new_0();
        let current_hour = now.get_hours() as usize;
        let current_minute = now.get_minutes() as usize;
        current_hour * 2 + if current_minute >= 30 { 1 } else { 0 }
    };

    let today_day = Date::new_0().get_day() as u8;

    rsx! {
        div { class: "schedule-chart",
            for (idx , value) in data.iter().enumerate() {
                {
                    let height = match value {
                        1 => 33,
                        2 => 66,
                        3 => 100,
                        _ => 0,
                    };
                    let mut bar_class = "schedule-bar".to_string();
                    if selected_day == today_day && idx == today_idx {
                        bar_class.push_str(" schedule-bar-current");
                    }
                    let hour = idx / 2;
                    let minute = (idx % 2) * 30;
                    let label = match value {
                        1 => "Low",
                        2 => "Medium",
                        3 => "High",
                        _ => "Off",
                    };
                    let is_hovered = *hovered.read() == Some(idx);

                    rsx! {
                        button {
                            key: "{idx}",
                            class: "{bar_class}",
                            style: "--bar-height: {height}%; position: relative;",
                            onclick: move |_| on_select.call(idx),
                            onmouseenter: move |_| hovered.set(Some(idx)),
                            onmouseleave: move |_| hovered.set(None),
                            span { class: "sr-only", "slot {idx}" }
                            if is_hovered {
                                {
                                    let tooltip_style = if idx >= 36 {
                                        "position: absolute; right: 110%; top: 50%; transform: translateY(-50%); min-width: 120px; background: #fff; color: #333; border: 1px solid #ccc; border-radius: 4px; padding: 6px 10px; box-shadow: 0 2px 8px rgba(0,0,0,0.12); z-index: 10; white-space: nowrap;"
                                    } else {
                                        "position: absolute; left: 110%; top: 50%; transform: translateY(-50%); min-width: 120px; background: #fff; color: #333; border: 1px solid #ccc; border-radius: 4px; padding: 6px 10px; box-shadow: 0 2px 8px rgba(0,0,0,0.12); z-index: 10; white-space: nowrap;"
                                    };
                                    rsx! {
                                        span { class: "schedule-tooltip", style: "{tooltip_style}",
                                            strong { "{hour:02}:{minute:02}" }
                                            br {}
                                            "{value} ({label})"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
