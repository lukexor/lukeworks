//! Error pages.

use leptos::{component, view, IntoView};
use leptos_dom::Fragment;

/// Error copy.
#[derive(Debug)]
#[must_use]
pub struct Error {
    pub heading: &'static str,
    pub message: &'static str,
    pub action: fn() -> Fragment,
}

/// 404 Not Found page.
#[component]
pub fn NotFound() -> impl IntoView {
    view! {  "404 Not Found" }
}

/// 500 Internal Server Error page.
#[component]
pub fn InternalServerError() -> impl IntoView {
    view! {  "500 Internal Server Error" }
}
