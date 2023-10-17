//! Portfolio Header components.

use crate::{
    portfolio::{
        data::{
            Route, DARK_COLOR_SCHEME, DARK_THEME_COLOR, LAYOUT, LIGHT_COLOR_SCHEME,
            LIGHT_THEME_COLOR, ROUTES,
        },
        icons::{DynIconButton, GitHubIcon, LinkedInIcon},
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
            href={ROUTES.home.path}
            title={ROUTES.home.title}
        >
            <span class="font-mono text-blue-500">"❱"</span>
            "L"
        </a>
    }
}

/// Portfolio navigation menu item.
#[component]
fn NavItem(route: &'static Route) -> impl IntoView {
    view! {
        <li class="mt-8 lg:mt-0 lg:mx-4">
            <a href=route.path on:click=|_| scroll_to_id(route.path)>
                {route.title}
            </a>
        </li>
    }
}

/// Portfolio navigation component.
#[component]
fn Nav() -> impl IntoView {
    let (menu_expanded, set_menu_expanded) = create_signal(false);
    let collapse = move |_| set_menu_expanded.set(false);
    let toggle_menu =
        move |_| update!(|set_menu_expanded| *set_menu_expanded = !*set_menu_expanded);

    // TODO: Ensure tab indexing is working correctly across links, both hidden and not

    view! {
        <span class="relative lg:hidden z-20">
            <button
                class="icon-link fa-solid fa-bars text-xl mx-2"
                class=("fa-times", menu_expanded)
                class=("!text-2xl", menu_expanded)
                on:click=toggle_menu
            />
        </span>
        <div
            class="absolute lg:relative top-0 right-0 w-0 max-w-md lg:w-auto transition-[width] h-full lg:h-auto bg-gray-700 lg:bg-transparent overflow-hidden z-10 lg:z-0"
            class=("!w-3/4", menu_expanded)
        >
            <ul class="flex flex-col lg:flex-row items-center lg:items-center px-6 py-3 lg:p-0">
                <NavItem route={&ROUTES.about} on:click=collapse />
                <NavItem route={&ROUTES.blog} on:click=collapse />
                <NavItem route={&ROUTES.projects} on:click=collapse />
                <NavItem route={&ROUTES.contact} on:click=collapse />
                <li class="mt-8 lg:hidden">
                    <div class="flex">
                        <DarkModeToggle />
                        <GitHubIcon />
                        <LinkedInIcon />
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
    let toggle_search = move |_| {
        update!(|set_search_expanded| *set_search_expanded = !*set_search_expanded);
        focus_search();
    };

    let (query, set_query) = create_signal(String::new());
    let clear_search = move |_| {
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

    view! {
        <div>
            <button
                class="icon-link fa-solid fa-search mx-2 text-xl lg:text-base"
                on:click=toggle_search
            />
            <div
                class="absolute lg:relative flex lg:inline-flex top-12 right-4 lg:top-[unset] lg:right-[unset] items-center w-0 transition-[width] overflow-hidden"
                class=("!w-72", search_expanded)
            >
                <input
                    _ref={search_ref}
                    type="text"
                    placeholder={LAYOUT.search_placeholder}
                    class="p-0 pl-2 pr-8 w-72 border bg-gray-100 dark:bg-gray-700 text-blue-500 dark:text-blue-400 border-blue-600 focus:border-red-400 dark:focus:border-red-500 focus-ring-red-400 dark:focus:ring-red-500"
                    tabindex=search_tabindex
                    prop:value=query
                    on:input=on_search_input
                />
                <button
                    class="icon-link fa-solid fa-times absolute lg:relative right-1 lg:right-6 text-lg"
                    tabindex=search_tabindex
                    on:click=clear_search
                />
            </div>
            <div
                class="border-b-2 border-dotted border-blue-500 bg-gray-100 dark:bg-gray-700 invisible absolute mt-10 p-6 left-0 w-full"
                class=("!visible", move || with!(|results, search_expanded| !results.is_empty() && *search_expanded))
            >
                <p>{move || {
                    let count = with!(|results| results.len());
                    format!(r#"{count} result{} for "{}":"#, if count > 1 { "s" } else { "" }, query.get())
                }}</p>
                <For
                    each=move || results.get()
                    key=|result| result.clone()
                    let:result
                >
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

    use_context::<RequestParts>()
        .map(|parts| {
            let cookies = CookieJar::from_headers(&parts.headers);
            if let Some(cookie) = cookies.get("darkmode") {
                cookie.value() == "true"
            } else {
                false
            }
        })
        .unwrap_or(false)
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

    let toggle_dark_class = move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            use wasm_bindgen::JsCast;
            let doc = document().unchecked_into::<web_sys::HtmlDocument>();
            if let Some(html) = doc.document_element() {
                if prefers_dark() {
                    let _ = html.class_list().remove_1(DARK_COLOR_SCHEME);
                } else {
                    let _ = html.class_list().add_1(DARK_COLOR_SCHEME);
                }
            }
        }
    };

    let color_scheme = move || {
        if prefers_dark() {
            DARK_COLOR_SCHEME
        } else {
            LIGHT_COLOR_SCHEME
        }
    };
    let theme_color = move || {
        if prefers_dark() {
            DARK_THEME_COLOR
        } else {
            LIGHT_THEME_COLOR
        }
    };
    let icon = move || {
        if prefers_dark() {
            LAYOUT.icons.dark_mode.icon
        } else {
            LAYOUT.icons.light_mode.icon
        }
    };

    view! {
        <Meta name="color-scheme" content=color_scheme />
        <Meta name="theme-color" content=theme_color />
        <ActionForm action=toggle_dark_mode>
            <input
                type="hidden"
                name="prefers_dark"
                value=move || !prefers_dark()
            />
            <DynIconButton
                icon=icon
                title="Toggle dark mode"
                on_click=toggle_dark_class
            />
        </ActionForm>
    }
}

/// Portfolio header component.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-between px-4 lg:px-12 py-2 bg-gray-200 dark:bg-gray-800">
            <Logo />
            <div class="hidden lg:flex items-center">
                <Nav />
                <SearchField />
                <DarkModeToggle />
                <GitHubIcon />
                <LinkedInIcon />
            </div>
            <div class="flex lg:hidden items-center">
                <SearchField />
                <Nav />
            </div>
        </header>
    }
}
