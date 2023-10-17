//! About page.

use crate::portfolio::data::ABOUT;
use leptos::*;

/// About page.
#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="w-full max-w-prose py-16">
            <h2 id="about" class="font-display text-4xl my-6 text-blue-500">{ABOUT.title}</h2>
            <div inner_html={ABOUT.content} />
        </div>
    }
}
