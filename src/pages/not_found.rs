//! Not found page.

use crate::lukeworks::{ROUTES, SUPPORT_EMAIL};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

/// Page not found for invalid routes.
#[component]
pub fn NotFound() -> impl IntoView {
    // Without this the response is a 200 carrying a "not found" page, which
    // tells crawlers the URL is real. Server-side only; there is no status code
    // to set once the document is already in the browser.
    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
        response.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Not Found" />
        <div class="py-20 text-center">
            <p class="mb-4 font-mono text-[13px] text-primary">"$ 404 Segmentation Fault"</p>
            <h1 class="mb-4 font-mono text-4xl font-bold tracking-tighter">
                "The page you're looking for is out of bounds."
            </h1>
            <p class="text-ink-dim">
                "Check the URL, file a " <a href=format!("mailto:{SUPPORT_EMAIL}")>"bug report"</a>
                ", or " <A href=ROUTES.home>"head back home"</A> "."
            </p>
        </div>
    }
}
