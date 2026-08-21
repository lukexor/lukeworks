//! The TetaNES emulator, playable in the page.
//!
//! The emulator is a prebuilt bundle under `public/tetanes-web/`, framed rather
//! than mounted: it ships its own document, styles and fonts. Two posts link
//! here, and so does the projects listing.
//!
//! Deliberately bare until Phase 4 rebuilds this page around the emulator
//! rather than around its document. Chrome of our own on top of chrome of its
//! own reads as two pages stacked, so the frame carries none: no heading, no
//! border, no background, and the page's gutters cancelled so it runs edge to
//! edge. Its palette is its own and does not match the site yet.

use leptos::prelude::*;
use leptos_meta::Title;

/// Where the bundle's own document lives, under the site root.
const BUNDLE: &str = "/tetanes-web/index.html";

/// The emulator page.
#[component]
pub fn TetanesWeb() -> impl IntoView {
    view! {
        <Title text="TetaNES" />

        // Tall enough to clear the bundle's whole document, which is an 880x696
        // canvas over a controls table. A frame shorter than its content grows
        // a scrollbar inside the page's own, and `scrolling="no"` is not
        // expressible here: leptos has no typed attribute for it.
        //
        // `gamepad` has to be granted explicitly, since a cross-document frame
        // gets no controller access by default.
        <iframe
            src=BUNDLE
            title="TetaNES"
            allow="gamepad *; fullscreen"
            class="block -my-10 -mx-6 w-screen border-0 sm:-mx-14 h-[1500px] max-w-[100vw]"
        ></iframe>
    }
}
