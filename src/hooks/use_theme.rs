//! Dark/light theme.
//!
//! Under islands, `#[component]` bodies never execute in the browser, so the
//! old approach — an `Effect` at the app root syncing `<body>` — could not work:
//! the effect would only ever run during server rendering. Responsibility is
//! split three ways instead:
//!
//! 1. The **server** resolves the theme from the `prefers-dark` cookie and
//!    renders it straight into the `<body>` class. No flash, no JS, and it is
//!    correct on the very first paint for anyone who has toggled before.
//! 2. A tiny **inline script** covers the one case the server cannot know: a
//!    visitor with no cookie whose OS prefers light. It runs before first paint.
//! 3. [`ThemeToggle`] is an `#[island]` — the only part that ships to the
//!    browser — which flips the class and persists the choice.
//!
//! Default is dark, which is why the no-cookie no-JS path needs no fallback.

/// Cookie used to persist the visitor's colour-scheme preference.
pub const PREFERS_DARK_COOKIE: &str = "prefers-dark";

/// Runs before first paint to apply the OS preference when no cookie is set.
///
/// Only ever *removes* `dark`: the server already renders the dark class, and
/// dark is the default, so light is the only case needing correction. Keeping
/// it to one branch means no flash in the common path.
pub const NO_FLASH_SCRIPT: &str = "\
if(!document.cookie.includes('prefers-dark')\
&&window.matchMedia('(prefers-color-scheme: light)').matches){\
document.body.classList.remove('dark');\
document.body.style.colorScheme='light';}";

/// Resolve the visitor's preference during server rendering.
///
/// Reads the `prefers-dark` cookie, defaulting to dark. Nothing else is
/// consulted: the previous implementation also read a `sec-ch-prefers-color-scheme`
/// request header, but that is a client hint the browser only sends once the
/// server has advertised `Accept-CH`, which this server never did. That branch
/// could therefore never fire, and its absence was masked by falling through to
/// the same dark default. The inline script above covers the case properly.
#[cfg(feature = "ssr")]
#[must_use]
pub fn prefers_dark_from_request() -> bool {
    use leptos::prelude::use_context;

    let Some(parts) = use_context::<axum::http::request::Parts>() else {
        return true;
    };
    let Some(cookies) = parts
        .headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
    else {
        return true;
    };

    cookies
        .split(';')
        .filter_map(|cookie| cookie.split_once('='))
        .find(|(name, _)| name.trim() == PREFERS_DARK_COOKIE)
        .is_none_or(|(_, value)| value.trim() != "false")
}

/// Body class carrying the server-resolved theme.
///
/// Tailwind's dark variant is `&:where(.dark, .dark *)`, driven by this class.
#[must_use]
pub const fn body_class(prefers_dark: bool) -> &'static str {
    if prefers_dark { "dark" } else { "" }
}
