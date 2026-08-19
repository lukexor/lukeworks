//! Blog listing.

use crate::components::post_list::{PostKind, PostList};
use leptos::prelude::*;
use leptos_meta::Title;

/// Every published blog post, grouped by year.
#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Title text="Blog" />

        <header class="mb-10">
            <p class="mb-3 font-mono text-[13px] text-primary">"$ ls -lt blog/"</p>
            <h1 class="mb-3 font-mono text-4xl font-bold tracking-tighter">"Blog"</h1>
            <p class="max-w-2xl leading-relaxed text-ink-dim">
                "Notes on emulation, Rust, and the parts of this trade that took me longest to
                learn."
            </p>
        </header>

        <PostList kind=PostKind::Blog />
    }
}
