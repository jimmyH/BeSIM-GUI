use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ScheduleChartProps {
    pub data: Vec<u8>,
    pub on_select: Callback<usize>,
    pub selected_day: u8,
}

#[function_component(ScheduleChart)]
pub fn schedule_chart(props: &ScheduleChartProps) -> Html {
    let hovered = use_state(|| None::<usize>);

    use js_sys::Date;
    let today_idx = {
        let now = Date::new_0();
        let current_hour = now.get_hours() as usize;
        let current_minute = now.get_minutes() as usize;
        current_hour * 2 + if current_minute >= 30 { 1 } else { 0 }
    };

    let today_day = Date::new_0().get_day() as u8;

    let bars = props
        .data
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            let height = match value {
                1 => 33,
                2 => 66,
                3 => 100,
                _ => 0,
            };
            let on_select = props.on_select.clone();
            let onclick = Callback::from(move |_| on_select.emit(idx));
            let hovered_set = hovered.clone();
            let onmouseover = Callback::from(move |_| hovered_set.set(Some(idx)));
            let hovered_clear = hovered.clone();
            let onmouseout = Callback::from(move |_| hovered_clear.set(None));
            let mut bar_class = "schedule-bar".to_string();
            // Only highlight if selected_day is today
            if props.selected_day == today_day && idx == today_idx {
                bar_class.push_str(" schedule-bar-current");
            }
            html! {
                <button
                    class={bar_class}
                    style={format!("--bar-height: {}%; position: relative;", height)}
                    {onclick}
                    {onmouseover}
                    {onmouseout}
                >
                    <span class="sr-only">{ format!("slot {}", idx) }</span>
                    { if *hovered == Some(idx) {
                        let hour = idx / 2;
                        let minute = (idx % 2) * 30;
                        let label = match value {
                            1 => "Low",
                            2 => "Medium",
                            3 => "High",
                            _ => "Off",
                        };
                        let tooltip_style = if idx >= 36 {
                            // Last quarter: show tooltip to the left
                            "position: absolute; right: 110%; top: 50%; transform: translateY(-50%); min-width: 120px; background: #fff; color: #333; border: 1px solid #ccc; border-radius: 4px; padding: 6px 10px; box-shadow: 0 2px 8px rgba(0,0,0,0.12); z-index: 10; white-space: nowrap;"
                        } else {
                            // Default: show tooltip to the right
                            "position: absolute; left: 110%; top: 50%; transform: translateY(-50%); min-width: 120px; background: #fff; color: #333; border: 1px solid #ccc; border-radius: 4px; padding: 6px 10px; box-shadow: 0 2px 8px rgba(0,0,0,0.12); z-index: 10; white-space: nowrap;"
                        };
                        html! {
                            <span class="schedule-tooltip" style={tooltip_style}>
                                <strong>{format!("{:02}:{:02}", hour, minute)}</strong><br/>
                                {format!("{} ({})", value, label)}
                            </span>
                        }
                    } else { html! {} } }
                </button>
            }
        })
        .collect::<Html>();

    html! { <div class="schedule-chart">{ bars }</div> }
}
