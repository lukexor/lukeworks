//! Portfolio Header components.

use crate::{
    portfolio::{
        constants::{colors, layout, routes},
        icons::{self, DynIconButton, GitHubIcon, LinkedInIcon},
    },
    scroll_to_id,
};
use leptos::*;
use leptos_dom::{create_node_ref, helpers::debounce, html::Input};
use leptos_meta::Meta;
use leptos_router::ActionForm;
use std::time::Duration;

/// Portfolio Logo component.
#[component]
fn Logo() -> impl IntoView {
    view! {
        <a
            id="logo"
            class="text-3xl text-red-400 font-display font-bold"
            href=routes::HOME
            title=layout::menu::HOME
        >
            <span class="font-mono text-blue-500">"❱"</span>
            {layout::LOGO}
        </a>
    }
}

/// Portfolio navigation menu item.
#[component]
fn NavItem(href: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <li class="mt-8 lg:mt-0 lg:mx-4">
            <a href=href on:click=|_| scroll_to_id(href)>
                {title}
            </a>
        </li>
    }
}

/// Portfolio navigation component.
#[component]
fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);
    let handle_menu_close = move |_| set_menu_open.set(false);
    let handle_menu_toggle = move |_| update!(|set_menu_open| *set_menu_open = !*set_menu_open);

    // TODO: Ensure tab indexing is working correctly across links, both hidden and not

    view! {
        <span class="relative lg:hidden z-20">
            <button
                class="icon-link fa-solid fa-bars text-xl mx-2"
                class=("fa-times", menu_open)
                class=("!text-2xl", menu_open)
                on:click=handle_menu_toggle
            ></button>
        </span>
        <div
            class="absolute lg:relative top-0 right-0 w-0 lg:w-auto max-w-md h-screen transition-[width] h-full lg:h-auto bg-gray-700 lg:bg-transparent overflow-hidden z-10 lg:z-0"
            class=("!w-3/4", menu_open)
        >
            <ul class="flex flex-col lg:flex-row items-center lg:items-center px-6 py-3 lg:p-0">
                <NavItem href=routes::HOME_ABOUT title=layout::menu::ABOUT on:click=handle_menu_close/>
                <NavItem href=routes::HOME_BLOG title=layout::menu::BLOG on:click=handle_menu_close/>
                <NavItem href=routes::HOME_PROJECTS title=layout::menu::PROJECTS on:click=handle_menu_close/>
                <NavItem href=routes::HOME_CONTACT title=layout::menu::CONTACT on:click=handle_menu_close/>
                <li class="mt-8 lg:hidden">
                    <div class="flex">
                        <DarkModeToggle/>
                        <GitHubIcon/>
                        <LinkedInIcon/>
                    </div>
                </li>
            </ul>
        </div>
    }
}

/// Search field component.
#[component]
fn SearchField() -> impl IntoView {
    let search_ref = create_node_ref::<Input>();

    let (search_expanded, set_search_expanded) = create_signal(false);
    let (results, set_results) = create_signal(vec![]);
    let focus_search = move || {
        search_ref
            .get()
            .and_then(|search_ref| search_ref.focus().ok())
            .unwrap_or_else(|| {
                logging::error!("failed to focus search field");
            });
    };
    let handle_search_toggle = move |_| {
        update!(|set_search_expanded| *set_search_expanded = !*set_search_expanded);
        focus_search();
    };

    let (query, set_query) = create_signal(String::new());
    let handle_clear_search = move |_| {
        set_query.set(String::new());
        set_results.set(vec![]);
        focus_search();
    };

    let mut search = debounce(Duration::from_millis(300), move |query: String| {
        // TODO: Query entries
        if !query.is_empty() {
            set_results.set(vec![query]);
        } else {
            set_results.set(vec![]);
        }
    });
    let on_search_input = move |ev| {
        let value = event_target_value(&ev);
        set_query.set(value.clone());
        search(value);
    };

    let search_tabindex = move || if search_expanded.get() { "" } else { "-1" };
    let show_results =
        move || with!(|results, search_expanded| !results.is_empty() && *search_expanded);
    let results_len = move || with!(|results| results.len());
    let query_str = move || query.get();

    view! {
        <div>
            <button
                class="icon-link fa-solid fa-search mx-2 text-xl lg:text-base"
                on:click=handle_search_toggle
            ></button>
            <div
                class="absolute z-10 lg:relative flex lg:inline-flex top-12 right-4 lg:top-[unset]
                lg:right-[unset] items-center w-0 transition-[width] overflow-hidden"
                class=("!w-72", search_expanded)
            >
                <input
                    node_ref=search_ref
                    type="text"
                    placeholder=layout::search::PLACEHOLDER
                    class="p-0 pl-2 pr-8 w-72 border bg-gray-100 dark:bg-gray-700 text-blue-500
                        dark:text-blue-400 border-blue-600 focus:border-red-400
                        dark:focus:border-red-500 focus-ring-red-400 dark:focus:ring-red-500"
                    tabindex=search_tabindex
                    prop:value=query
                    on:input=on_search_input
                />
                <button
                    class="icon-link fa-solid fa-times absolute lg:relative right-1 lg:right-6 text-lg"
                    tabindex=search_tabindex
                    on:click=handle_clear_search
                ></button>
            </div>
            <div
                class="border-b-2 border-dotted border-blue-500 bg-gray-100 dark:bg-gray-700
                invisible absolute z-10 mt-10 lg:mt-4 p-6 left-0 w-full"
                class=("!visible", show_results)
            >

                <p>{layout::search::result_summary(results_len, query_str)}</p>
                <For each=move || results.get() key=|result| result.clone() let:result>
                    <div>{result}</div>
                </For>
            </div>
        </div>
    }
}

#[cfg(not(feature = "ssr"))]
pub fn initial_prefers_dark() -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    if cookie.contains("darkmode=") {
        cookie.contains("darkmode=true")
    } else {
        window()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map_or(false, |m| m.matches())
    }
}

#[cfg(feature = "ssr")]
pub fn initial_prefers_dark() -> bool {
    use axum_extra::extract::cookie::CookieJar;
    use leptos_axum::RequestParts;

    let default_prefers_dark = true;
    use_context::<RequestParts>()
        .and_then(|parts| {
            CookieJar::from_headers(&parts.headers)
                .get("darkmode")
                .map(|c| c.value() == "true")
        })
        .unwrap_or(default_prefers_dark)
}

#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError> {
    use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;
    response.overwrite(response_parts);

    Ok(prefers_dark)
}

/// A dark mode toggle.
#[component]
fn DarkModeToggle() -> impl IntoView {
    let initial = initial_prefers_dark();

    let toggle_dark_mode = create_server_action::<ToggleDarkMode>();
    let input = toggle_dark_mode.input();
    let value = toggle_dark_mode.value();
    let prefers_dark = move || match (input.get(), value.get()) {
        (Some(input), _) => input.prefers_dark,
        (_, Some(Ok(value))) => value,
        _ => initial,
    };

    #[cfg(feature = "hydrate")]
    create_effect(move |_| {
        use wasm_bindgen::JsCast;
        let doc = document().unchecked_into::<web_sys::HtmlDocument>();
        if let Some(html) = doc.document_element() {
            if prefers_dark() {
                let _ = html.class_list().add_1("dark");
            } else {
                let _ = html.class_list().remove_1("dark");
            }
        }
    });

    let color_scheme = move || {
        if prefers_dark() {
            "dark"
        } else {
            "light"
        }
    };
    let theme_color = move || {
        if prefers_dark() {
            colors::DARK_THEME
        } else {
            colors::LIGHT_THEME
        }
    };
    let icon = move || {
        if prefers_dark() {
            icons::MOON
        } else {
            icons::SUN
        }
    };

    view! {
        <Meta name="color-scheme" content=color_scheme/>
        <Meta name="theme-color" content=theme_color/>
        <ActionForm action=toggle_dark_mode>
            <input type="hidden" name="prefers_dark" value=move || (!prefers_dark()).to_string()/>
            <DynIconButton icon=icon type_="submit" title=layout::icons::DARK_MODE />
        </ActionForm>
    }
}

/// Portfolio header component.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="absolute w-full z-[1] flex justify-between px-4 lg:px-12 py-2">
            <Logo/>
            <div class="hidden lg:flex items-center">
                <Nav/>
                <SearchField/>
                <DarkModeToggle/>
                <GitHubIcon/>
                <LinkedInIcon/>
            </div>
            <div class="flex lg:hidden items-center">
                <SearchField/>
                <Nav/>
            </div>
        </header>
    }
}
