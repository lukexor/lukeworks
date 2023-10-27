//! Icon components.

use crate::portfolio::constants::{layout, links, routes};
use leptos::*;

pub const GITHUB: &str = "fa-brands fa-github";
pub const LINKEDIN: &str = "fa-brands fa-linkedin";
pub const RSS: &str = "fa-solid fa-rss";
pub const EMAIL: &str = "fa-solid fa-envelope";
pub const MOON: &str = "fa-solid fa-moon";
pub const SUN: &str = "fa-solid fa-sun";
pub const CHEVRON_DOWN: &str = "fa-solid fa-circle-chevron-down";

/// Icon with an href link.
#[component]
pub fn IconLink(
    icon_class: &'static str,
    href: &'static str,
    title: &'static str,
) -> impl IntoView {
    view! {
        <a
            class="icon-link text-xl mx-2"
            class=icon_class
            href=href
            title=title
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
        <IconLink icon_class=GITHUB href=links::GITHUB title=layout::icons::GITHUB/>
    }
}

/// LinkedIn icon.
#[component]
pub fn LinkedInIcon() -> impl IntoView {
    view! {
        <IconLink icon_class=LINKEDIN href=links::LINKEDIN title=layout::icons::LINKEDIN/>
    }
}

/// RSS icon.
#[component]
pub fn RssIcon() -> impl IntoView {
    view! {
        <IconLink icon_class=RSS href=routes::RSS title=layout::icons::RSS/>
    }
}

/// Email icon.
#[component]
pub fn EmailIcon() -> impl IntoView {
    view! {
        <IconLink icon_class=EMAIL href=links::EMAIL title=layout::icons::EMAIL/>
    }
}
