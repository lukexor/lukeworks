//! About page.

use crate::portfolio::constants::homepage::about;
use leptos::*;

/// About page.
#[component]
pub fn About() -> impl IntoView {
    view! {
        <section>
            <h2 id="about" class="font-display text-4xl text-blue-500">
                {about::HEADING}
            </h2>
            <p inner_html=about::BODY/>
        </section>
    }
}
