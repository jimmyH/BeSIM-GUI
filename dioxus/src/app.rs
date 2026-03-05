use dioxus::prelude::*;
use dioxus_router::components::*;
use gloo_timers::future::TimeoutFuture;
use js_sys::Date;

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

fn format_time(date: &Date) -> String {
    let hours = date.get_hours();
    let minutes = date.get_minutes();
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
    let mut now = use_signal(Date::new_0);

    use_future(move || async move {
        loop {
            TimeoutFuture::new(5000).await;
            now.set(Date::new_0());
        }
    });

    let current_date = now.read();
    let day = day_name(current_date.get_day());
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
