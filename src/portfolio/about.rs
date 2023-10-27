//! About page.

use crate::portfolio::constants::homepage::about;
use leptos::*;

/// About page.
#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="w-full max-w-prose py-16">
            <h2 id="about" class="font-display text-4xl my-6 text-blue-500">
                {about::HEADING}
            </h2>
            <p inner_html=about::BODY/>
        </div>
    }
}
