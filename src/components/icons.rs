//! Inline SVG icons.
//!
//! Hand-written rather than pulled from an icon crate: the site needs six
//! glyphs, and each is a single path that ships as markup with no dependency
//! and no lookup table. Every icon inherits `currentColor` and sizes from the
//! `class` the caller passes, so one definition serves the header, the footer
//! and the contact list.

use leptos::prelude::*;

/// Every icon is drawn in outline, so none of them takes a fill.
const FILL: &str = "none";

/// GitHub.
#[component]
pub fn GithubIcon(#[prop(into, default = "size-[18px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="1.8"
            aria-hidden="true"
        >
            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.9a3.4 3.4 0 0 0-.9-2.6c3-.3 6.2-1.5 6.2-6.7A5.2 5.2 0 0 0 19.9 5 4.9 4.9 0 0 0 19.8.5S18.5.1 15.5 2a13.4 13.4 0 0 0-7 0C5.5.1 4.2.5 4.2.5A4.9 4.9 0 0 0 4.1 5a5.2 5.2 0 0 0-1.4 3.8c0 5.2 3.2 6.4 6.2 6.7a3.4 3.4 0 0 0-.9 2.6V22" />
        </svg>
    }
}

/// LinkedIn.
#[component]
pub fn LinkedInIcon(#[prop(into, default = "size-[18px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="1.8"
            aria-hidden="true"
        >
            <rect x="2" y="9" width="4" height="12" />
            <circle cx="4" cy="4" r="2" />
            <path d="M10 21V9m0 4a4 4 0 0 1 8 0v8" />
        </svg>
    }
}

/// RSS.
#[component]
pub fn RssIcon(#[prop(into, default = "size-[18px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="1.8"
            aria-hidden="true"
        >
            <path d="M4 11a9 9 0 0 1 9 9M4 4a16 16 0 0 1 16 16" />
            <circle cx="5" cy="19" r="1.5" fill="currentColor" />
        </svg>
    }
}

/// Email.
#[component]
pub fn MailIcon(#[prop(into, default = "size-[18px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="1.8"
            aria-hidden="true"
        >
            <rect x="2" y="4" width="20" height="16" rx="2" />
            <path d="m2 7 10 6 10-6" />
        </svg>
    }
}

/// Magnifier, for the header search affordance.
#[component]
pub fn SearchIcon(#[prop(into, default = "size-[13px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="2"
            aria-hidden="true"
        >
            <circle cx="11" cy="11" r="7" />
            <path d="m20 20-4.2-4.2" />
        </svg>
    }
}

/// Sun, shown by the theme toggle while the dark theme is active.
#[component]
pub fn SunIcon(#[prop(into, default = "size-4".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="1.9"
            aria-hidden="true"
        >
            <circle cx="12" cy="12" r="4.5" />
            <path d="M12 2v2m0 16v2M2 12h2m16 0h2M5 5l1.5 1.5M17.5 17.5 19 19M19 5l-1.5 1.5M6.5 17.5 5 19" />
        </svg>
    }
}

/// Moon, shown by the theme toggle while the light theme is active.
#[component]
pub fn MoonIcon(#[prop(into, default = "size-4".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="1.9"
            aria-hidden="true"
        >
            <path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z" />
        </svg>
    }
}

/// Rightwards arrow, for calls to action and forward navigation.
///
/// Drawn rather than written as `→`, which neither vendored font subset covers.
/// A text arrow falls back to a system font mid-string and lands at a different
/// weight and width than the run around it.
#[component]
pub fn ArrowIcon(#[prop(into, default = "size-[15px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="2.4"
            aria-hidden="true"
        >
            <path d="M5 12h14m-6-7 7 7-7 7" />
        </svg>
    }
}

/// Leftwards arrow, for back links and previous-post navigation.
///
/// Drawn for the same reason as [`ArrowIcon`].
#[component]
pub fn BackArrowIcon(#[prop(into, default = "size-[15px]".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill=FILL
            stroke="currentColor"
            stroke-width="2.4"
            aria-hidden="true"
        >
            <path d="M19 12H5m6-7-7 7 7 7" />
        </svg>
    }
}
