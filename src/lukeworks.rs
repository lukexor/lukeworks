use crate::{
    components::theme_toggle::ThemeToggle,
    hooks::use_theme,
    pages::{blog::Blog, home::Home, not_found::NotFound, post::Post, projects::Projects},
};
use leptos::{either::Either, prelude::*};
use leptos_meta::{
    HashedStylesheet, Link, Meta, MetaTags, Stylesheet, Title, provide_meta_context,
};
use leptos_router::{
    ParamSegment, StaticSegment,
    components::{A, FlatRoutes, Route, Router},
};

/// Support email.
pub const SUPPORT_EMAIL: &str = "me@lukeworks.tech";
/// Application routes.
pub const ROUTES: AppRoutes = AppRoutes {
    home: "/",
    about: "/about",
    blog: "/blog",
    projects: "/projects",
};

/// Type for application routes.
#[derive(Debug)]
#[must_use]
pub struct AppRoutes {
    pub home: &'static str,
    pub about: &'static str,
    pub blog: &'static str,
    pub projects: &'static str,
}

/// Renders either a HashedStylesheet or Stylesheet based on configured option for `hash_files`.
/// Set LEPTOS_HASH_FILES=true for release builds.
#[component]
fn RootStylesheet(options: LeptosOptions) -> impl IntoView {
    if options.hash_files {
        Either::Left(view! { <HashedStylesheet id="style" options /> })
    } else {
        let href = format!(
            "{pkg_path}/{css_name}.css",
            pkg_path = &options.site_pkg_dir,
            css_name = &options.output_name
        );
        Either::Right(view! { <Stylesheet id="style" href /> })
    }
}

/// HTML shell with metadata and reload scripts.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    // Resolved on the server so the correct theme is in the very first byte of
    // HTML. The WASM build has no request to read and never renders the shell.
    #[cfg(feature = "ssr")]
    let prefers_dark = use_theme::prefers_dark_from_request();
    #[cfg(not(feature = "ssr"))]
    let prefers_dark = true;

    let color_scheme = if prefers_dark { "dark" } else { "light" };

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta
                    name="description"
                    content="
                    A blog and project portfolio by Lucas Petherbridge on programming,
                    technology, and video games.
                    "
                />

                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <RootStylesheet options />
                <MetaTags />

                // Server-rendered rather than synced by an effect: <body> and
                // this tag both live outside the reactive tree.
                <Meta name="color-scheme" content=color_scheme />

                <Link rel="icon" href="/favicon.ico" />
                <Link rel="manifest" href="/site.webmanifest" />
            // The rel="alternate" feed link belongs here, but goes in with the
            // /rss route in Phase 4 rather than advertising a 404 until then.
            </head>
            <body class=use_theme::body_class(prefers_dark) style=format!("color-scheme:{color_scheme}")>
                // Corrects the one case the server cannot know: no cookie, and
                // the OS prefers light. Inline and before the body content so it
                // lands ahead of first paint.
                <script>{use_theme::NO_FLASH_SCRIPT}</script>
                <LukeWorks />
            </body>
        </html>
    }
}

/// Main entrypoint with global context and layout.
#[component]
pub fn LukeWorks() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title formatter=move |text| format!("{text} — Lucas Petherbridge | Software Engineer") />

        // Header lives inside <Router> so its nav links are client-side
        // navigations rather than full page loads.
        <Router>
            <Header />
            <main>
                <FlatRoutes transition=true fallback=NotFound>
                    <Route path=StaticSegment("") view=Home />
                    <Route path=StaticSegment("/about") view=About />
                    <Route path=StaticSegment("/blog") view=Blog />
                    <Route path=StaticSegment("/projects") view=Projects />
                    // Must stay last: a bare param segment matches any single
                    // path segment, including the static routes above.
                    <Route path=ParamSegment("post") view=Post />
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <p id="panic-error" class="hidden self-center">
            "An internal error occurred. Try refreshing the page or file a "
            <a href=format!("mailto:{SUPPORT_EMAIL}")>"bug report"</a>
            "."
        </p>
        <header class="flex items-center justify-between gap-4 p-4">
            <A href=ROUTES.home attr:class="font-bold">
                "LukeWorks"
            </A>
            <nav class="flex items-center gap-4">
                <A href=ROUTES.blog>"Blog"</A>
                <A href=ROUTES.projects>"Projects"</A>
                <A href=ROUTES.about>"About"</A>
                <ThemeToggle />
            </nav>
        </header>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! { <div>"About"</div> }
}

#[component]
pub fn Contact() -> impl IntoView {
    view! { <div>"Contact"</div> }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! { <div>"Footer"</div> }
}
