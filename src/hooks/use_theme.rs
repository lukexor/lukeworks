use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::use_cookie;

/// Reactive theme.
#[derive(Copy, Clone)]
pub struct Theme {
    /// Preference for dark mode.
    pub prefers_dark: Signal<bool>,
    /// Toggle preference for dark mode.
    pub toggle_prefers_dark: Callback<()>,
}

/// Get prefers dark preference.
pub fn use_prefers_dark() -> Signal<bool> {
    with_context::<Theme, _>(|theme| theme.prefers_dark).expect("valid Theme context")
}

/// Returns theme reflecting the the users preference for dark mode.
///
/// `leptos_use` has `use_preferred_dark` and `use_media_query` hooks, but neither of those support
/// setting the "default" as Dark mode.
pub fn use_theme() -> Theme {
    if let Some(theme) = use_context::<Theme>() {
        return theme;
    }

    // Cookie used to track users color scheme preference.
    pub const PREFERS_SCHEME_COOKIE: &str = "prefers-dark";

    // Get initial preference from media-query or HTTP headers, defaulting to true
    #[cfg(not(feature = "ssr"))]
    let initial_prefers_dark = window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap_or(None)
        .is_none_or(|query| query.matches());
    #[cfg(feature = "ssr")]
    let initial_prefers_dark = {
        use axum::http::request;
        let headers = use_context::<request::Parts>().map(|parts| parts.headers);
        let prefers_scheme = headers
            .and_then(|headers| headers.get("sec-ch-prefers-color-scheme").cloned())
            .map(|header| header.to_str().unwrap_or_default().to_owned());
        matches!(prefers_scheme.as_deref(), None | Some("dark"))
    };
    // Get cookie preference, if previously set
    let (prefers_dark_cookie, set_prefers_dark_cookie) =
        use_cookie::<bool, FromToStringCodec>(PREFERS_SCHEME_COOKIE);

    // Use cookie, or default to initial value based on headers or defaulting to dark
    let prefers_dark =
        Signal::derive(move || prefers_dark_cookie.get().unwrap_or(initial_prefers_dark));

    // Toggling preference sets a cookie to persist.
    let toggle_prefers_dark = move || {
        let new_prefers_dark = !prefers_dark.get();
        set_prefers_dark_cookie.set(Some(new_prefers_dark));
    };

    Effect::new(move |_| {
        let color_scheme = if prefers_dark.get() { "dark" } else { "light" };

        // Fixes issue toggling theme after page is loaded to update meta tags which aren't reactive
        if let Ok(Some(el)) = document().query_selector("meta[name=\"color-scheme\"]") {
            let _ = el.set_attribute("content", color_scheme);
        }

        // Since body is outside App, we need to sync color-scheme on it as well
        if let Some(el) = document()
            .get_elements_by_tag_name("body")
            .get_with_index(0)
        {
            let _ = el.style(("color-scheme", color_scheme));

            let class_list = el.class_list();
            if prefers_dark.get() {
                let _ = class_list.add_1("dark");
            } else {
                let _ = class_list.remove_1("dark");
            }
        }
    });

    let theme = Theme {
        prefers_dark,
        toggle_prefers_dark: toggle_prefers_dark.into(),
    };
    // Allow other parts of the app to react to theme changes.
    provide_context(theme);

    theme
}
