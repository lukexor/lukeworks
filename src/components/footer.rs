//! Site footer.

use crate::components::social_links::SocialLinks;
use chrono::{Datelike, Utc};
use leptos::prelude::*;

/// Copyright line and the social row, on every page.
///
/// The year comes from the clock rather than a constant so the notice does not
/// quietly go stale in January.
#[component]
pub fn Footer() -> impl IntoView {
    let year = Utc::now().year();

    view! {
        <footer class="mt-20 border-t border-rule">
            <div class="flex flex-col gap-4 justify-between items-center py-8 px-6 mx-auto max-w-6xl sm:flex-row sm:px-14">
                <span class="font-mono text-xs text-ink-dim">
                    "© " {year} " Luke Petherbridge · All Rights Reserved"
                </span>
                <SocialLinks />
            </div>
        </footer>
    }
}
