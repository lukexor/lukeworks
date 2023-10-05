//! Portfolio Header components.

use leptos::{component, view, IntoView};

/// Portfolio Logo component.
#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="text-3xl dark:text-blue-500 font-monospace font-semibold">
            <span class="dark:text-red-400">"❱"</span>
            "L"
        </div>
    }
}

/// Portfolio navigation component.
#[component]
fn Nav() -> impl IntoView {
    view! {
        <ul>
            <li><a href="/">"Home"</a></li>
        </ul>
    }
}

/// Portfolio header component.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <Logo />
        <Nav />
    }
}
