//! Error pages.

use leptos::{component, view, IntoView, Scope};

#[component]
pub fn NotFound(_cx: Scope) -> impl IntoView {
    view! { cx, "404 Not Found" }
}

#[component]
pub fn InternalServerError(_cx: Scope) -> impl IntoView {
    view! { cx, "500 Internal Server Error" }
}
