use gloo_timers::callback::Interval;
use js_sys::Date;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, Route};

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

#[function_component(App)]
pub fn app() -> Html {
    let now = use_state(Date::new_0);

    {
        let now = now.clone();
        use_effect(move || {
            let handle = Interval::new(5000, move || {
                now.set(Date::new_0());
            });
            move || drop(handle)
        });
    }

    let day = day_name(now.get_day());
    let time = format_time(&now);

    html! {
        <BrowserRouter>
            <div class="wrapper">
                <header class="header">
                    <div class="toolbar">
                        <div class="logo">{ "BeSMART" }</div>
                        <nav class="nav-links">
                            <Link<Route> classes="nav-link" to={Route::Devices}>
                                { "Devices" }
                            </Link<Route>>
                            <Link<Route> classes="nav-link" to={Route::About}>
                                { "About" }
                            </Link<Route>>
                        </nav>
                        <div class="spacer" />
                        <div class="clock">{ format!("{} {}", day, time) }</div>
                    </div>
                </header>
                <main class="child">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}
