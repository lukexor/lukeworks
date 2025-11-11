pub mod lukeworks;
pub mod components {
    pub mod button;
    pub mod image;
}
pub mod hooks {
    pub mod use_theme;
}
pub mod pages {
    pub mod blog;
    pub mod error;
    pub mod home;
    pub mod not_found;
    pub mod post;
    pub mod projects;
    pub mod resume;
    pub mod search;
    pub mod tetanes_web;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::lukeworks::LukeWorks;
    use std::panic;

    panic::set_hook(Box::new(|info: &panic::PanicHookInfo<'_>| {
        let error_div = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("panic-error"));
        if let Some(error_div) = error_div
            && let Err(err) = error_div.class_list().remove_1("hidden")
        {
            leptos::logging::error!("failed to add panic error: {err:?}");
        }

        console_error_panic_hook::hook(info);
    }));
    leptos::mount::hydrate_body(LukeWorks);
}
