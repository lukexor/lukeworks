//! The TetaNES emulator, playable in the page.
//!
//! The emulator is a prebuilt bundle under `public/tetanes-web/`, framed rather
//! than mounted: it ships its own document, styles and fonts. Two posts link
//! here, and so does the projects listing.
//!
//! Deliberately bare until Phase 4 rebuilds this page around the emulator
//! rather than around its document. Chrome of our own on top of chrome of its
//! own reads as two pages stacked, so the frame carries none: no heading, no
//! blurb, no border, no background.
//!
//! The bundle's palette is its own, patched to follow the site's light and dark
//! schemes: it reads the `prefers-dark` cookie before its first paint, and
//! [`crate::components::theme_toggle::ThemeToggle`] reaches into the frame to
//! keep the two in step on a click.

use leptos::prelude::*;
use leptos_meta::{Body, Title};

/// Where the bundle's own document lives, under the site root.
const BUNDLE: &str = "/tetanes-web/index.html";

/// The frame's id, which [`size_frame_to_content`] looks it up by.
const FRAME_ID: &str = "tetanes-frame";

/// The class `<main>` drops its measure for, defined in `style/tailwind.css`.
const WIDE_BODY: &str = "frame-page";

/// Match the frame's height to the document inside it.
///
/// The emulator's canvas grows with the scale the visitor picks, so one height
/// either clips the taller scales or leaves a gap under the shorter ones. The
/// bundle is same-origin, which puts its height within reach the way
/// [`crate::components::theme_toggle`] reaches its root element.
#[cfg(feature = "hydrate")]
fn size_frame_to_content(frame: &web_sys::HtmlIFrameElement) {
    use wasm_bindgen::JsCast;

    let Some(root) = frame
        .content_document()
        .and_then(|document| document.document_element())
    else {
        return;
    };
    // The root element's own box, not `scroll_height`, which is that floored by
    // the frame it is being measured in. Floored, it can only ever grow: a
    // visitor moving from 5x back to 3x would keep the taller frame and a gap
    // under the canvas.
    let root: &web_sys::HtmlElement = root.unchecked_ref();
    let height = root.offset_height();
    if height <= 0 {
        return;
    }

    let frame: &web_sys::HtmlElement = frame.unchecked_ref();
    let wanted = format!("{height}px");
    // Writing the height resizes the frame, which resizes the document being
    // measured, so an unconditional write feeds the observer its own output.
    if frame.style().get_property_value("height").as_deref() != Ok(wanted.as_str()) {
        let _ = frame.style().set_property("height", &wanted);
    }
}

/// Size the frame once it has a document, and again whenever that document
/// changes height.
#[cfg(feature = "hydrate")]
fn watch_frame() {
    use wasm_bindgen::{JsCast, closure::Closure};

    let Some(frame) = document()
        .get_element_by_id(FRAME_ID)
        .and_then(|element| element.dyn_into::<web_sys::HtmlIFrameElement>().ok())
    else {
        return;
    };

    // The frame may already have loaded by the time this runs, in which case no
    // further `load` fires and the first measurement has to happen here.
    size_frame_to_content(&frame);

    let on_load = {
        let frame = frame.clone();
        Closure::<dyn Fn()>::new(move || size_frame_to_content(&frame))
    };
    let _ = frame.add_event_listener_with_callback("load", on_load.as_ref().unchecked_ref());
    on_load.forget();

    let observer = {
        let frame = frame.clone();
        Closure::<dyn Fn()>::new(move || size_frame_to_content(&frame))
    };
    if let Ok(resize) = web_sys::ResizeObserver::new(observer.as_ref().unchecked_ref()) {
        if let Some(root) = frame
            .content_document()
            .and_then(|document| document.document_element())
        {
            resize.observe(&root);
        }
        observer.forget();
        std::mem::forget(resize);
    }
}

/// The emulator page.
#[component]
pub fn TetanesWeb() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| watch_frame());

    view! {
        <Title text="TetaNES" />
        // Drops `<main>`'s measure for this route. The canvas is
        // `scale * 8/7 * 256` across and the bundle centres its body at
        // `max-width: 80%`, so 5x wants 1829px where `max-w-6xl` leaves 922px.
        <Body attr:class=WIDE_BODY />

        // `w-full` is the body's width, which already excludes the scrollbar,
        // where `100vw` would overflow it. The height is a starting point that
        // `size_frame_to_content` replaces with the document's own.
        //
        // `gamepad` has to be granted explicitly, since a cross-document frame
        // gets no controller access by default.
        <iframe
            id=FRAME_ID
            src=BUNDLE
            title="TetaNES"
            allow="gamepad *; fullscreen"
            class="block -my-10 w-full border-0 h-[1500px]"
        ></iframe>
    }
}
