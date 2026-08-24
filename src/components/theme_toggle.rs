//! Dark/light toggle.

use crate::{
    components::icons::{MoonIcon, SunIcon},
    hooks::use_theme::PREFERS_DARK_COOKIE,
};
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Mirror the colour scheme onto every same-origin frame on the page.
///
/// A framed document has an `<html>` of its own, so the class this toggle
/// writes cannot reach it through CSS. `/tetanes-web/` is served from this
/// origin, which puts its root element within reach directly and saves
/// standing up a message channel to carry one boolean.
///
/// Frames take `light` where the page takes `dark`, because each document
/// defaults to dark on its own: a palette that hung off the *absence* of a
/// class would paint the frame light between parse and script.
///
/// A cross-origin frame yields `None` here rather than throwing, and the
/// sketch frames, black whatever the page does, take the class and ignore it.
fn sync_frames(prefers_dark: bool) {
    let Ok(frames) = document().query_selector_all("iframe") else {
        return;
    };
    for index in 0..frames.length() {
        let Some(frame) = frames
            .get(index)
            .and_then(|node| node.dyn_into::<web_sys::HtmlIFrameElement>().ok())
        else {
            continue;
        };
        let Some(root) = frame
            .content_document()
            .and_then(|frame_document| frame_document.document_element())
        else {
            continue;
        };

        let _ = if prefers_dark {
            root.class_list().remove_1("light")
        } else {
            root.class_list().add_1("light")
        };
        let root: web_sys::HtmlElement = root.unchecked_into();
        let _ = root
            .style()
            .set_property("color-scheme", if prefers_dark { "dark" } else { "light" });
    }
}

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

        // After the cookie, so a frame that reloads mid-click reads the new
        // value rather than the old one.
        sync_frames(prefers_dark);
    };

    view! {
        <button
            type="button"
            on:click=toggle
            // `p-2` around an 18px icon keeps the hit target at 34px square,
            // the same box the footer's social icons carry.
            class="p-2 rounded cursor-pointer text-ink-dim hover:text-accent"
            aria-label="Toggle dark mode"
            title="Toggle dark mode"
        >
            // Which icon shows is driven by CSS, not state, so there is no
            // second copy of the theme to fall out of sync with the class
            // on <html>. Drawn rather than set as ☀/☾ text, which the latin
            // font subsets do not cover and which fell back to whatever glyph
            // the system happened to have.
            <span class="hidden dark:inline">
                <SunIcon />
            </span>
            <span class="inline dark:hidden">
                <MoonIcon />
            </span>
        </button>
    }
}
