//! Blog Pages.

use leptos::{component, view, IntoView};

/// Blog Summary.
#[component]
pub fn BlogSummary() -> impl IntoView {
    view! {
        <section>
            "Blog Summary"
        </section>
    }
}

/// Blog Listing.
#[component]
pub fn Blog() -> impl IntoView {
    view! {
        "Blog"
    }
}
