//! Portfolio Header components.

use crate::portfolio::data::{LAYOUT, ROUTES};
use leptos::{
    component, create_signal, event_target_value, view, For, IntoAttribute, IntoView, SignalGet,
    SignalSet,
};
use leptos_dom::helpers::debounce;
use std::time::Duration;

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
        <ul class="flex items-center">
            <li class="mr-8"><a href=ROUTES.about.path>{ROUTES.about.title}</a></li>
            <li class="mr-8"><a href=ROUTES.projects.path>{ROUTES.projects.title}</a></li>
            <li class="mr-8"><a href=ROUTES.blog.path>{ROUTES.blog.title}</a></li>
            <li class="mr-8"><a href=ROUTES.contact.path>{ROUTES.contact.title}</a></li>
        </ul>
    }
}

/// Search field component.
#[component]
fn SearchField() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (results, set_results) = create_signal::<Vec<String>>(vec![]);
    let mut search = debounce(Duration::from_millis(200), move |query| {
        leptos::logging::log!("{query}");
        set_results.set(vec!["hi".into()]);
    });
    view! {
        <div class="">
            <input
                type="text"
                placeholder={LAYOUT.search_placeholder}
                class="h-6 bg-gray-700 text-blue-400 focus:border-red-500 focus:shadow-sm focus:shadow-red-500 focus:ring-red-500 focus:ring-offset-red-500"
                prop:value=query.get()
                on:input=move |ev| {
                    leptos::logging::log!("{}", event_target_value(&ev));
                    set_query.set(event_target_value(&ev));
                    search(event_target_value(&ev))
                }
                // value={value}
                // onChange={handleChange}
                // onFocus={() => setExpanded(true)}
                // onBlur={() => setExpanded(false)}
            />
            <For
                each=move || results.get()
                key=|result| result.clone()
                let:result
            >
                <div>{result}</div>
            </For>
        </div>
    }
}

/// Portfolio header component.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-between">
            <Logo />
            <div class="flex items-center">
                <Nav />
                <SearchField />
            </div>
        </header>
    }
}
