use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ScheduleChartProps {
    pub data: Vec<u8>,
    pub on_select: Callback<usize>,
}

#[function_component(ScheduleChart)]
pub fn schedule_chart(props: &ScheduleChartProps) -> Html {
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
            html! {
                <button class="schedule-bar" style={format!("--bar-height: {}%", height)} {onclick}>
                    <span class="sr-only">{format!("slot {}", idx)}</span>
                </button>
            }
        })
        .collect::<Html>();

    html! {
        <div class="schedule-chart">{bars}</div>
    }
}
