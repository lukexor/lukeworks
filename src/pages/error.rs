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
            <ul>
                <For
                    each=move || errors.get()
                    key=|(error_id, _)| error_id.clone()
                    children=move |(_, error)| {
                        view! { <li>{error.to_string()}</li> }
                    }
                />
            </ul>
            <p>
                "Check the URL, file a " <a href=format!("email:{SUPPORT_EMAIL}")>bug report</a>
                ", or " <A href=ROUTES.home>"head back home"</A>"."
            </p>
        </div>
    }
}
