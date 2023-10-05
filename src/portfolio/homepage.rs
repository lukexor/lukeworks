//! Portfolio homepage.

use crate::portfolio::data::IconAttrs;
use leptos::{component, view, IntoView};

#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Intro {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub action: IconAttrs,
}

/// Homepage.
#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        "Homepage"
    }
}
