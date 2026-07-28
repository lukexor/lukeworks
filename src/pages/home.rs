use leptos::prelude::*;
use leptos_meta::Title;

/// Homepage.
#[component]
pub fn Home() -> impl IntoView {
    // Every other route sets one; without it `/` rendered no <title> element at
    // all, so the tab showed the bare URL.
    view! { <Title text="Home" /> }
}
