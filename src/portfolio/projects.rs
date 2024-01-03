//! Projects.

use leptos::{component, view, IntoView};

/// Project Summary.
#[component]
pub fn ProjectsSummary() -> impl IntoView {
    view! {
        <section>
            "Projects Summary"
        </section>
    }
}

/// Project Listing.
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        "Projects"
    }
}
