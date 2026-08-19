//! Project listing.

use crate::components::post_list::{PostKind, PostList};
use leptos::prelude::*;
use leptos_meta::Title;

/// Every published project, grouped by year.
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="Projects" />

        <header class="mb-10">
            <p class="mb-3 font-mono text-[13px] text-primary">"$ ls -lt projects/"</p>
            <h1 class="mb-3 font-mono text-4xl font-bold tracking-tighter">"Things I've built"</h1>
            <p class="max-w-2xl leading-relaxed text-ink-dim">
                "Emulators, graphics libraries, and a pile of sketches that exist because the idea
                would not leave me alone."
            </p>
        </header>

        <PostList kind=PostKind::Project />
    }
}
