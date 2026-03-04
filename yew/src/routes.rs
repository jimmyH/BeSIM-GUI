use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/about")]
    About,
    #[at("/devices")]
    Devices,
    #[at("/devices/:id")]
    DeviceDetails { id: String },
    #[at("/devices/:device_id/rooms")]
    Rooms { device_id: String },
    #[at("/devices/:device_id/rooms/:room_id")]
    RoomDetails { device_id: String, room_id: String },
    #[at("/devices/:device_id/rooms/:room_id/history")]
    RoomHistory { device_id: String, room_id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Root => html! { <Redirect<Route> to={Route::Devices} /> },
        Route::About => html! { <pages::AboutPage /> },
        Route::Devices => html! { <pages::DevicesPage /> },
        Route::DeviceDetails { id } => html! { <pages::DeviceDetailsPage id={id} /> },
        Route::Rooms { device_id } => html! { <pages::RoomsPage device_id={device_id} /> },
        Route::RoomDetails { device_id, room_id } => {
            html! { <pages::RoomDetailsPage device_id={device_id} room_id={room_id} /> }
        }
        Route::RoomHistory { device_id, room_id } => {
            html! { <pages::RoomHistoryPage device_id={device_id} room_id={room_id} /> }
        }
        Route::NotFound => html! { <pages::NotFoundPage /> },
    }
}
