//! The one place the social and contact URLs are written down.
//!
//! Rendered by the footer and the about page, so a changed handle is a one-line
//! edit rather than a hunt through several components.

use crate::{
    components::icons::{GithubIcon, LinkedInIcon, MailIcon, RssIcon},
    lukeworks::SUPPORT_EMAIL,
};
use leptos::prelude::*;

/// Profile URL on GitHub.
pub const GITHUB_URL: &str = "https://github.com/lukexor";
/// Profile URL on LinkedIn.
pub const LINKEDIN_URL: &str = "https://linkedin.com/in/lucaspetherbridge";
/// The site's own feed, served by the `/rss` handler.
pub const RSS_URL: &str = "/rss";

/// Icon-only row, for the footer.
#[component]
pub fn SocialLinks(#[prop(into, default = "gap-5".into())] class: String) -> impl IntoView {
    view! {
        <ul class=format!("flex items-center {class}")>
            <li>
                <a
                    href=GITHUB_URL
                    rel="me noopener"
                    title="GitHub"
                    class="block text-ink-dim hover:text-accent"
                >
                    <GithubIcon />
                    <span class="sr-only">GitHub</span>
                </a>
            </li>
            <li>
                <a
                    href=LINKEDIN_URL
                    rel="me noopener"
                    title="LinkedIn"
                    class="block text-ink-dim hover:text-accent"
                >
                    <LinkedInIcon />
                    <span class="sr-only">LinkedIn</span>
                </a>
            </li>
            <li>
                <a href=RSS_URL title="RSS feed" class="block text-ink-dim hover:text-accent">
                    <RssIcon />
                    <span class="sr-only">"RSS feed"</span>
                </a>
            </li>
            <li>
                <a
                    href=format!("mailto:{SUPPORT_EMAIL}")
                    title="Email"
                    class="block text-ink-dim hover:text-accent"
                >
                    <MailIcon />
                    <span class="sr-only">Email</span>
                </a>
            </li>
        </ul>
    }
}

/// Labelled cards carrying the handle as well as the icon, for the about page
/// where the address itself is the useful part.
#[component]
pub fn ContactCards() -> impl IntoView {
    view! {
        <ul class="grid gap-3.5 sm:grid-cols-2">
            <ContactCard href=GITHUB_URL label="GitHub" handle="github.com/lukexor">
                <GithubIcon class="size-[21px]" />
            </ContactCard>
            <ContactCard href=LINKEDIN_URL label="LinkedIn" handle="lucaspetherbridge">
                <LinkedInIcon class="size-[21px]" />
            </ContactCard>
            <ContactCard href=format!("mailto:{SUPPORT_EMAIL}") label="Email" handle=SUPPORT_EMAIL>
                <MailIcon class="size-[21px]" />
            </ContactCard>
            <ContactCard href=RSS_URL label="RSS" handle=RSS_URL>
                <RssIcon class="size-[21px]" />
            </ContactCard>
        </ul>
    }
}

#[component]
fn ContactCard(
    #[prop(into)] href: String,
    #[prop(into)] label: String,
    #[prop(into)] handle: String,
    children: Children,
) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                rel="noopener"
                class="flex gap-3.5 items-center py-4 px-5 no-underline rounded border hover:no-underline border-rule bg-panel text-ink hover:border-accent"
            >
                {children()}
                <span>
                    <span class="block text-sm font-semibold">{label}</span>
                    <span class="block font-mono text-xs text-ink-dim">{handle}</span>
                </span>
            </a>
        </li>
    }
}
