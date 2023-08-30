//! lukeworks.tech

#![warn(clippy::all, clippy::pedantic)]
// Too many false positives with auto-generated leptos types
#![allow(clippy::module_name_repetitions, clippy::used_underscore_binding)]

use cfg_if::cfg_if;

pub mod components;
#[cfg(feature = "ssr")]
pub mod file_server;
pub mod lukeworks;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use leptos::{view, warn};
        use wasm_bindgen::prelude::wasm_bindgen;
        use lukeworks::LukeWorks;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(|cx| {
                view! { cx,  <LukeWorks /> }
            });
        }
    }
}
