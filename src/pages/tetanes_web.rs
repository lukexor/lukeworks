//! The TetaNES emulator, playable in the page.
//!
//! The emulator is a prebuilt bundle under `public/tetanes-web/`, framed rather
//! than mounted: it ships its own document, styles and fonts. Two posts link
//! here, and so does the projects listing.

use leptos::prelude::*;
use leptos_meta::Title;

/// Where the bundle's own document lives, under the site root.
const BUNDLE: &str = "/tetanes-web/index.html";

/// The emulator page.
#[component]
pub fn TetanesWeb() -> impl IntoView {
    view! {
        <Title text="TetaNES" />

        <p class="mb-3 font-mono text-[13px] text-primary">"$ tetanes --web"</p>
        <h1 class="mb-4 font-mono text-4xl font-bold tracking-tighter">"TetaNES"</h1>
        <p class="mb-8 max-w-2xl leading-relaxed text-ink-dim">
            "A cycle-accurate NES emulator written in Rust, compiled to WebAssembly. "
            "Load a ROM to play it here, or read " <a href="/tetanes">"how it was built"</a> "."
        </p>

        // The bundle sizes its canvas at 880x696 and puts a controls table
        // under it, so the frame is tall and scrolls internally rather than
        // being resized from out here. `gamepad` has to be granted explicitly:
        // a cross-document frame gets no controller access by default.
        <iframe
            src=BUNDLE
            title="TetaNES"
            allow="gamepad *; fullscreen"
            class="w-full rounded border h-[80vh] min-h-[600px] border-rule bg-panel"
        ></iframe>
    }
}
