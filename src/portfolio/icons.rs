//! Icon components.

use crate::portfolio::data::{IconLinkAttrs, LAYOUT};
use leptos::*;

/// Icon with an href link.
#[component]
pub fn IconLink(attrs: IconLinkAttrs) -> impl IntoView {
    view! {
        <a
            class="icon-link text-xl mx-2"
            class=attrs.icon
            href=attrs.href
            title=attrs.title
        />
    }
}

/// Icon button with on:click and dynamic icon.
#[component]
pub fn DynIconButton<I>(icon: I, title: &'static str, type_: &'static str) -> impl IntoView
where
    I: Fn() -> &'static str + 'static,
{
    view! {
        <button
            type=type_
            class=move || format!("icon-link {} text-xl mx-2 w-[20px]", icon())
            title=title
        />
    }
}

/// GitHub icon.
#[component]
pub fn GitHubIcon() -> impl IntoView {
    view! {
        <IconLink attrs=LAYOUT.icons.github />
    }
}

/// LinkedIn icon.
#[component]
pub fn LinkedInIcon() -> impl IntoView {
    view! {
        <IconLink attrs=LAYOUT.icons.linkedin />
    }
}

/// RSS icon.
#[component]
pub fn RssIcon() -> impl IntoView {
    view! {
        <IconLink attrs=LAYOUT.icons.rss />
    }
}

/// Email icon.
#[component]
pub fn EmailIcon() -> impl IntoView {
    view! {
        <IconLink attrs=LAYOUT.icons.email />
    }
}
