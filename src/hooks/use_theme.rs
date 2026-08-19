//! Dark/light theme.
//!
//! `<html>` is rendered by `shell` and lives outside the reactive tree, so the
//! theme is not driven by a signal. Responsibility is split three ways:
//!
//! 1. The **server** resolves the theme from the `prefers-dark` cookie and
//!    renders it straight into the `<html>` class. No flash, no JS, and it is
//!    correct on the very first paint for anyone who has toggled before.
//! 2. A tiny **inline script** covers the one case the server cannot know: a
//!    visitor with no cookie whose OS prefers light. It runs before first paint.
//! 3. [`crate::components::theme_toggle::ThemeToggle`] flips the class on click
//!    and persists the choice.
//!
//! Default is dark, which is why the no-cookie no-JS path needs no fallback.
//!
//! The class hangs off `<html>` rather than `<body>` because of hydration:
//! `hydrate_body` starts its cursor at `<body>`'s first child, so *any* node
//! rendered ahead of the app root — the no-flash script included — desyncs the
//! walk and panics with "expected a marker node, but found this instead:
//! script". Keeping the script in `<head>` means it cannot touch `<body>`,
//! which does not exist yet when it runs, so the class moved up with it.

/// Cookie used to persist the visitor's colour-scheme preference.
pub const PREFERS_DARK_COOKIE: &str = "prefers-dark";

/// Runs before first paint to apply the OS preference when no cookie is set.
///
/// Lives in `<head>` and therefore targets `document.documentElement` —
/// `document.body` has not been parsed yet at that point. See the module docs
/// for why it cannot simply be moved into `<body>`.
///
/// Only ever *removes* `dark`: the server already renders the dark class, and
/// dark is the default, so light is the only case needing correction. Keeping
/// it to one branch means no flash in the common path.
pub const NO_FLASH_SCRIPT: &str = "\
if(!document.cookie.includes('prefers-dark')\
&&window.matchMedia('(prefers-color-scheme: light)').matches){\
document.documentElement.classList.remove('dark');\
document.documentElement.style.colorScheme='light';}";

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
    parts
        .headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .is_none_or(prefers_dark_from_cookie_header)
}

/// Parse a `Cookie:` header value. Split out from the request plumbing so the
/// precedence rules are testable without standing up a server.
///
/// Dark unless the cookie is explicitly `false`, so a missing or malformed
/// cookie lands on the site's default rather than flipping the theme.
#[must_use]
pub fn prefers_dark_from_cookie_header(cookies: &str) -> bool {
    cookies
        .split(';')
        .filter_map(|cookie| cookie.split_once('='))
        .find(|(name, _)| name.trim() == PREFERS_DARK_COOKIE)
        .is_none_or(|(_, value)| value.trim() != "false")
}

/// `<html>` class carrying the server-resolved theme.
///
/// Tailwind's dark variant is `&:where(.dark, .dark *)`, driven by this class.
#[must_use]
pub const fn root_class(prefers_dark: bool) -> &'static str {
    if prefers_dark { "dark" } else { "" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_dark() {
        assert!(prefers_dark_from_cookie_header(""));
        assert!(prefers_dark_from_cookie_header("other=1"));
        // Malformed values must not flip the theme.
        assert!(prefers_dark_from_cookie_header("prefers-dark"));
        assert!(prefers_dark_from_cookie_header("prefers-dark=wat"));
    }

    #[test]
    fn honours_an_explicit_light_preference() {
        assert!(!prefers_dark_from_cookie_header("prefers-dark=false"));
        assert!(!prefers_dark_from_cookie_header(
            "a=1; prefers-dark=false; b=2"
        ));
        // Whitespace around the pair is normal in a Cookie header.
        assert!(!prefers_dark_from_cookie_header(
            "a=1;  prefers-dark = false "
        ));
    }

    #[test]
    fn explicit_dark_stays_dark() {
        assert!(prefers_dark_from_cookie_header("prefers-dark=true"));
    }

    #[test]
    fn does_not_match_a_similarly_named_cookie() {
        // A prefix match here would let an unrelated cookie drive the theme.
        assert!(prefers_dark_from_cookie_header("prefers-dark-mode=false"));
        assert!(prefers_dark_from_cookie_header("xprefers-dark=false"));
    }

    #[test]
    fn root_class_tracks_preference() {
        assert_eq!(root_class(true), "dark");
        assert_eq!(root_class(false), "");
    }

    #[test]
    fn no_flash_script_stays_out_of_the_body() {
        // The script runs from <head>, where document.body is still null.
        // Reaching for it there is silently a no-op, so guard the regression.
        assert!(!NO_FLASH_SCRIPT.contains("document.body"));
        assert!(NO_FLASH_SCRIPT.contains("document.documentElement"));
    }
}
