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
            class="text-3xl text-blue-500 font-monospace font-semibold"
            href={ROUTES.home.path}
            title={ROUTES.home.title}
        >
            <span class="text-red-400">"❱"</span>
            "L"
        </a>
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
        <span class="relative md:hidden z-10">
            <button
                class="icon-link fa-solid fa-bars text-xl mx-2"
                class=("fa-times", menu_expanded)
                class=("!text-2xl", menu_expanded)
                on:click=toggle_menu
            />
        </span>
        <div
            class="absolute md:relative top-0 right-0 w-0 md:w-auto transition-[width] h-full md:h-auto bg-gray-700 md:bg-transparent overflow-hidden z-0"
            class=("!w-3/4", menu_expanded)
        >
            <ul class="flex flex-col md:flex-row items-center md:items-center px-6 py-3 md:p-0">
                <li class="mt-12 md:mt-0 md:mr-8"><a tabindex=menu_tabindex href=ROUTES.about.path>{ROUTES.about.title}</a></li>
                <li class="mt-8 md:mt-0 md:mr-8"><a tabindex=menu_tabindex href=ROUTES.projects.path>{ROUTES.projects.title}</a></li>
                <li class="mt-8 md:mt-0 md:mr-8"><a tabindex=menu_tabindex href=ROUTES.blog.path>{ROUTES.blog.title}</a></li>
                <li class="mt-8 md:mt-0 md:mr-8"><a tabindex=menu_tabindex href=ROUTES.contact.path>{ROUTES.contact.title}</a></li>
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
    let toggle_search =
        move |_| update!(|set_search_expanded| *set_search_expanded = !*set_search_expanded);

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
                class="icon-link fa-solid fa-search mx-2 md:mx-8 text-xl md:text-base"
                on:click=toggle_search
            />
            <div
                class="absolute flex right-6 top-12 md:top-16 md:right-20 items-center w-0 transition-[width] overflow-hidden"
                class=("!w-64", search_expanded)
            >
                <input
                    _ref={search_ref}
                    type="text"
                    placeholder={LAYOUT.search_placeholder}
                    class="p-0 pl-2 pr-8 border bg-gray-700 text-blue-400 border-blue-600 focus:border-red-500 focus:ring-red-500"
                    tabindex=search_tabindex
                    prop:value=query
                    on:input=on_search_input
                />
                <button
                    class="icon-link fa-solid fa-times absolute right-1 mr-2 text-lg"
                    tabindex=search_tabindex
                    on:click=clear_search
                />
            </div>
            <div
                class="border-b-2 border-dotted border-blue-500 bg-gray-700 invisible absolute mt-10 p-6 left-0 w-full"
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
fn initial_prefers_dark() -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    logging::log!("client cookie: {:?}", cookie);
    if cookie.contains("darkmode=") {
        cookie.contains("darkmode=true")
    } else {
        true
    }
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark() -> bool {
    use axum_extra::extract::cookie::CookieJar;
    use leptos_axum::RequestParts;

    use_context::<RequestParts>()
        .map(|parts| {
            let cookies = CookieJar::from_headers(&parts.headers);
            if let Some(cookie) = cookies.get("darkmode") {
                logging::log!("ssr cookie darkmode={:?}", cookie.value());
                cookie.value() == "true"
            } else {
                true
            }
        })
        .unwrap_or(true)
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

    let color_scheme = move || {
        if prefers_dark() {
            logging::log!("prefers_dark: true");
            "dark".to_string()
        } else {
            logging::log!("prefers_dark: false");
            "light".to_string()
        }
    };
    let icon = move || if prefers_dark() { "fa-moon" } else { "fa-sun" };

    view! {
        <Meta
            name="color-scheme"
            content=color_scheme
        />
        <ActionForm action=toggle_dark_mode>
            <input
                type="hidden"
                name="prefers_dark"
                value=move || (!prefers_dark()).to_string()
            />
            <button
                type="submit"
                class=move || format!("icon-link fa-solid {} text-xl mx-2", icon())
            />
        </ActionForm>
    }
}

/// Portfolio header component.
#[component]
pub fn Header() -> impl IntoView {
    let github_icon = LAYOUT.social_icons.github;
    view! {
        <header class="flex justify-between mb-12">
            <Logo />
            <div class="flex items-center">
                <SearchField />
                <Nav />
                <DarkModeToggle />
                <a
                    class="icon-link text-xl mx-2"
                    class={github_icon.icon}
                    href={github_icon.href}
                    title={github_icon.title}
                />
            </div>
        </header>
    }
}
