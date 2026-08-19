//! Dark/light toggle.

use crate::hooks::use_theme::PREFERS_DARK_COOKIE;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Toggles the colour scheme and persists it for a year.
///
/// Keeps no theme state of its own: the authority is the `dark`
/// class the server rendered onto `<html>`, which this reads on click. `<html>`
/// lives outside the reactive tree (it is written by `shell`), so driving it
/// from a signal would mean an effect syncing the two anyway — reading the DOM
/// keeps it correct regardless of how the initial value was arrived at: cookie,
/// inline script, or the dark default.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let toggle = move |_| {
        // `<html>` rather than `<body>` — see `use_theme`'s module docs.
        let Some(root) = document().document_element() else {
            return;
        };
        let prefers_dark = !root.class_list().contains("dark");

        let _ = if prefers_dark {
            root.class_list().add_1("dark")
        } else {
            root.class_list().remove_1("dark")
        };
        // `document_element` is an `Element`, which has no `style()`; the cast
        // is infallible for <html> and keeps `color-scheme` on the same node as
        // the class so the two can never disagree.
        let root: web_sys::HtmlElement = root.unchecked_into();
        let _ = root
            .style()
            .set_property("color-scheme", if prefers_dark { "dark" } else { "light" });

        // `SameSite=Lax` so it accompanies top-level navigations, which is what
        // lets the server render the right theme on the next request.
        let cookie =
            format!("{PREFERS_DARK_COOKIE}={prefers_dark}; path=/; max-age=31536000; SameSite=Lax");
        // `unchecked_into` rather than `dyn_ref`: the document is always an
        // HtmlDocument here, and a failed `dyn_ref` would silently skip
        // persisting the choice rather than surface anything.
        let document: web_sys::HtmlDocument = document().unchecked_into();
        let _ = document.set_cookie(&cookie);
    };

    view! {
        <button
            type="button"
            on:click=toggle
            class="p-2 rounded cursor-pointer"
            aria-label="Toggle dark mode"
            title="Toggle dark mode"
        >
            // Which glyph shows is driven by CSS, not state, so there is no
            // second copy of the theme to fall out of sync with the class
            // on <html>.
            <span class="hidden dark:inline">"☀"</span>
            <span class="inline dark:hidden">"☾"</span>
        </button>
    }
}
