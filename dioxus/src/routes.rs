use dioxus::prelude::*;
use dioxus_router::Routable;

use crate::app::Layout;
use crate::pages;

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Devices,
    #[route("/about")]
    About,
    #[route("/devices/:id")]
    DeviceDetails { id: String },
    #[route("/devices/:device_id/rooms")]
    Rooms { device_id: String },
    #[route("/devices/:device_id/rooms/:room_id")]
    RoomDetails { device_id: String, room_id: String },
    #[route("/devices/:device_id/rooms/:room_id/history")]
    RoomHistory { device_id: String, room_id: String },
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn Devices() -> Element {
    rsx! {
        pages::DevicesPage {}
    }
}

#[component]
fn About() -> Element {
    rsx! {
        pages::AboutPage {}
    }
}

#[component]
fn DeviceDetails(id: String) -> Element {
    rsx! {
        pages::DeviceDetailsPage { id }
    }
}

#[component]
fn Rooms(device_id: String) -> Element {
    rsx! {
        pages::RoomsPage { device_id }
    }
}

#[component]
fn RoomDetails(device_id: String, room_id: String) -> Element {
    rsx! {
        pages::RoomDetailsPage { device_id, room_id }
    }
}

#[component]
fn RoomHistory(device_id: String, room_id: String) -> Element {
    rsx! {
        pages::RoomHistoryPage { device_id, room_id }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        pages::NotFoundPage {}
    }
}
