//! Generic error page.

use crate::lukeworks::{ROUTES, SUPPORT_EMAIL};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

/// Error page for uncaught errors.
#[component]
pub fn Error(#[prop(into)] errors: Signal<Errors>) -> impl IntoView {
    view! {
        <Title text="Error" />
        <div class="flex flex-col items-center w-full">
            <h1 class="my-4 text-3xl">"Oops. An error occurred."</h1>
            <h2 class="text-xl">Errors:</h2>
            // Plain iteration rather than `<For>`: this page is rendered once on
            // the server and never re-runs, so keyed reconciliation buys nothing.
            // `<For>` would also require its key to be `Serialize` under islands,
            // which `ErrorId` is not.
            <ul>
                {move || {
                    errors
                        .get()
                        .into_iter()
                        .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                        .collect_view()
                }}
            </ul>
            <p>
                "Check the URL, file a " <a href=format!("mailto:{SUPPORT_EMAIL}")>bug report</a>
                ", or " <A href=ROUTES.home>"head back home"</A>"."
            </p>
        </div>
    }
}
