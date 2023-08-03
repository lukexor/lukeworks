//! lukeworks.tech

pub mod about;
pub mod blog;
pub mod contact;
pub mod editor;
pub mod homepage;
pub mod lukeworks;
pub mod post;
pub mod projects;
pub mod resume;
pub mod search;
pub mod tetanes_web;

#[cfg(feature = "hydrate")]
use leptos::*;
#[cfg(feature = "hydrate")]
use lukeworks::{LukeWorks, LukeWorksProps};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    // log!("hydrate mode - hydrating ({})", lukeworks::message());

    leptos::mount_to_body(|cx| {
        view! { cx,  <LukeWorks /> }
    });
}
