//! Contact page.

use leptos::{component, view, IntoView};

/// Contact copy.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Contact {
    pub name: &'static str,
    pub email: &'static str,
    pub message: &'static str,
}

/// Contact page.
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        "Contact"
    }
}
