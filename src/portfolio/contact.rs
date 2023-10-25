//! Contact page.

use crate::portfolio::data::CONTACT;
use leptos::*;

/// Contact page.
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="w-full max-w-prose py-16">
            <h2 id="contact" class="font-display text-4xl my-6 text-blue-500">{CONTACT.title}</h2>
            <p inner_html=CONTACT.content />
        </div>
    }
}
