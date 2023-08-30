//! Navigation

use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="m-5">
            <ul>
                <li><a href="/">"Home"</a></li>
            </ul>
        </div>
    }
}
