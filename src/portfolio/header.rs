//! Portfolio Header components.

use crate::portfolio::data::{LAYOUT, ROUTES};
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
            class="text-3xl text-blue-500 dark:text-blue-500 font-monospace font-semibold"
            href={ROUTES.home.path}
            title={ROUTES.home.title}
        >
            <span class="text-red-400 dark:text-red-400">"❱"</span>
            "L"
        </a>
    }
}

/// GitHub icon.
#[component]
fn GitHubIcon() -> impl IntoView {
    let github_icon = LAYOUT.social_icons.github;
    view! {
        <a
            class="icon-link text-xl mx-2"
            class={github_icon.icon}
            href={github_icon.href}
            title={github_icon.title}
        />
    }
}

/// Portfolio navigation component.
#[component]
fn Nav() -> impl IntoView {
    let (menu_expanded, set_menu_expanded) = create_signal(false);
    let toggle_menu =
        move |_| update!(|set_menu_expanded| *set_menu_expanded = !*set_menu_expanded);
    let menu_tabindex = move || if menu_expanded.get() { "" } else { "-1" };

    view! {
        <span class="relative lg:hidden z-10">
            <button
                class="icon-link fa-solid fa-bars text-xl mx-2"
                class=("fa-times", menu_expanded)
                class=("!text-2xl", menu_expanded)
                on:click=toggle_menu
            />
        </span>
        <div
            class="absolute lg:relative top-0 right-0 w-0 max-w-md lg:w-auto transition-[width] h-full lg:h-auto bg-gray-700 lg:bg-transparent overflow-hidden z-0"
            class=("!w-3/4", menu_expanded)
        >
            <ul class="flex flex-col lg:flex-row items-center lg:items-center px-6 py-3 lg:p-0">
                <li class="mt-12 lg:mt-0 lg:mr-8"><a tabindex=menu_tabindex href=ROUTES.about.path>{ROUTES.about.title}</a></li>
                <li class="mt-8 lg:mt-0 lg:mr-8"><a tabindex=menu_tabindex href=ROUTES.projects.path>{ROUTES.projects.title}</a></li>
                <li class="mt-8 lg:mt-0 lg:mr-8"><a tabindex=menu_tabindex href=ROUTES.blog.path>{ROUTES.blog.title}</a></li>
                <li class="mt-8 lg:mt-0 lg:mr-8"><a tabindex=menu_tabindex href=ROUTES.contact.path>{ROUTES.contact.title}</a></li>
                <li class="mt-8 lg:hidden">
                    <div class="flex">
                        <DarkModeToggle />
                        <GitHubIcon />
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

    let toggle_dark_class = move || {
        #[cfg(not(feature = "ssr"))]
        {
            use wasm_bindgen::JsCast;
            let doc = document().unchecked_into::<web_sys::HtmlDocument>();
            if let Some(html) = doc.document_element() {
                if prefers_dark() {
                    let _ = html.class_list().remove_1("dark");
                } else {
                    let _ = html.class_list().add_1("dark");
                }
            }
        }
    };

    let color_scheme = move || {
        if prefers_dark() {
            "dark"
        } else {
            "light"
        }
    };
    let theme_color = move || {
        if prefers_dark() {
            "#0b0d0a"
        } else {
            "#e5e9e1"
        }
    };
    let icon = move || {
        if prefers_dark() {
            "fa-moon"
        } else {
            "fa-sun"
        }
    };

    view! {
        <Meta
            name="color-scheme"
            content=color_scheme
        />
        <Meta
            name="theme-color"
            content=theme_color
        />
        <ActionForm action=toggle_dark_mode>
            <input
                type="hidden"
                name="prefers_dark"
                value=move || (!prefers_dark()).to_string()
            />
            <button
                type="submit"
                class=move || format!("icon-link fa-solid {} text-xl mx-2 w-[20px]", icon())
                on:click=move |_| toggle_dark_class()
            />
        </ActionForm>
    }
}

/// Portfolio header component.
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-between mb-12">
            <Logo />
            <div class="hidden lg:flex items-center">
                <Nav />
                <SearchField />
                <DarkModeToggle />
                <GitHubIcon />
            </div>
            <div class="flex lg:hidden items-center">
                <SearchField />
                <Nav />
            </div>
        </header>
    }
}
