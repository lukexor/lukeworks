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
use leptos_meta::Title;

/// Where the bundle's own document lives, under the site root.
const BUNDLE: &str = "/tetanes-web/index.html";

/// The emulator page.
#[component]
pub fn TetanesWeb() -> impl IntoView {
    view! {
        <Title text="TetaNES" />

        // Container width, not viewport width: the bundle centres its own body
        // at `max-width: 80%`, so a wider frame spreads an 880px canvas across
        // a page that was laid out for less.
        //
        // 1400px clears that canvas plus the controls table under it. A shorter
        // frame grows a scrollbar inside the page's own.
        //
        // `gamepad` has to be granted explicitly, since a cross-document frame
        // gets no controller access by default.
        <iframe
            src=BUNDLE
            title="TetaNES"
            allow="gamepad *; fullscreen"
            class="block -m-10 w-full border-0 h-[1500px]"
        ></iframe>
    }
}
