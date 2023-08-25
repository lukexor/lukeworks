//! lukeworks.tech

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)] // Too many false positives with auto-generated leptos types

use cfg_if::cfg_if;

mod lukeworks;

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
