//! Not found page.

use crate::lukeworks::{ROUTES, SUPPORT_EMAIL};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

/// Page not found for invalid routes.
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Not Found" />
        <div class="flex flex-col items-center w-full">
            <h1 class="my-4 text-3xl">"Page not found."</h1>
            <p>"The page you were looking for is not available"</p>
            <p>
                "Check the URL, file a " <a href=format!("email:{SUPPORT_EMAIL}")>bug report</a>
                ", or " <A href=ROUTES.home>"head back home"</A>"."
            </p>
        </div>
    }
}
