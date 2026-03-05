use dioxus::prelude::*;

use crate::api::{self, LocalClock};
use crate::routes::Route;

fn day_name(day: u32) -> &'static str {
    match day {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "",
    }
}

fn format_time(clock: &LocalClock) -> String {
    let hours = clock.hours;
    let minutes = clock.minutes;
    format!("{:02}:{:02}", hours, minutes)
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Layout() -> Element {
    let mut now = use_signal(api::local_clock);

    use_future(move || async move {
        loop {
            api::sleep(5000).await;
            now.set(api::local_clock());
        }
    });

    let current_date = now.read();
    let day = day_name(current_date.weekday);
    let time = format_time(&current_date);

    rsx! {
        div { class: "wrapper",
            header { class: "header",
                div { class: "toolbar",
                    div { class: "logo", "BeSMART" }
                    nav { class: "nav-links",
                        Link { to: Route::Devices, class: "nav-link", "Devices" }
                        Link { to: Route::About, class: "nav-link", "About" }
                    }
                    div { class: "spacer" }
                    div { class: "clock", "{day} {time}" }
                }
            }
            main { class: "child", Outlet::<Route> {} }
        }
    }
}
