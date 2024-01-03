//! Contact page.

use crate::portfolio::constants::homepage::contact;
use leptos::*;

/// Contact page.
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section>
            <h2 id="contact" class="font-display text-4xl my-6 text-blue-500">
                {contact::HEADING}
            </h2>
            <p inner_html=contact::BODY/>
        </section>
    }
}
