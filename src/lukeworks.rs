use crate::{
    components::{footer::Footer, icons::SearchIcon, theme_toggle::ThemeToggle},
    hooks::use_theme,
    pages::{
        about::About, blog::Blog, home::Home, not_found::NotFound, post::Post, projects::Projects,
        search::Search,
    },
};
use leptos::{either::Either, prelude::*};
use leptos_meta::{
    HashedStylesheet, Link, Meta, MetaTags, Stylesheet, Title, provide_meta_context,
};
use leptos_router::{
    ParamSegment, SsrMode, StaticSegment,
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
    search: "/search",
};

/// Type for application routes.
#[derive(Debug)]
#[must_use]
pub struct AppRoutes {
    /// Landing page.
    pub home: &'static str,
    /// Bio and contact details.
    pub about: &'static str,
    /// Blog listing, grouped by year.
    pub blog: &'static str,
    /// Project listing.
    pub projects: &'static str,
    /// Search results, driven by the `q` query parameter.
    pub search: &'static str,
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
        // Theme rides on <html>, not <body>: hydration walks <body>'s children
        // from the first one, so the app root has to be the only thing in there.
        <html
            lang="en"
            class=use_theme::root_class(prefers_dark)
            style=format!("color-scheme:{color_scheme}")
        >
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

                // Corrects the one case the server cannot know: no cookie, and
                // the OS prefers light. Inline and in <head> so it lands ahead
                // of first paint without displacing anything inside <body>.
                <script>{use_theme::NO_FLASH_SCRIPT}</script>

                <Link
                    rel="alternate"
                    type_="application/rss+xml"
                    title="Lucas Petherbridge"
                    href="/rss"
                />
            </head>
            <body>
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
            // The container lives here rather than per page so every route
            // shares one measure and one gutter. A page that wants the full
            // width (the homepage hero) breaks out with its own negative
            // margin instead of the container being optional.
            <main class="py-10 px-6 mx-auto max-w-6xl sm:px-14">
                <FlatRoutes transition=true fallback=NotFound>
                    <Route path=StaticSegment("") view=Home />
                    <Route path=StaticSegment("/about") view=About />
                    <Route path=StaticSegment("/search") view=Search />
                    // The three routes below resolve a server function before
                    // they have anything to show. Under the default out-of-order
                    // mode their `Suspense` streams into a trailing <template>
                    // and JS swaps it in, so <main> ships empty and a crawler or
                    // a reader without JS sees no post at all. Async holds the
                    // response until the resource resolves, which is
                    // microseconds against an in-memory table.
                    <Route path=StaticSegment("/blog") view=Blog ssr=SsrMode::Async />
                    <Route path=StaticSegment("/projects") view=Projects ssr=SsrMode::Async />
                    // Must stay last: a bare param segment matches any single
                    // path segment, including the static routes above.
                    <Route path=ParamSegment("post") view=Post ssr=SsrMode::Async />
                </FlatRoutes>
            </main>
            <Footer />
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
        <header class="border-b border-rule">
            <div class="flex gap-6 justify-between items-center py-4 px-6 mx-auto max-w-6xl sm:px-14">
                <A
                    href=ROUTES.home
                    attr:class="font-mono text-[17px] font-bold tracking-tight text-ink no-underline hover:no-underline"
                >
                    "luke"
                    <span class="text-primary">"works"</span>
                    <span class="text-ink-dim">".tech"</span>
                </A>
                <nav class="flex gap-5 items-center font-mono sm:gap-7 text-[13px]">
                    <A href=ROUTES.blog attr:class="text-ink-dim hover:text-accent">
                        "blog"
                    </A>
                    <A href=ROUTES.projects attr:class="text-ink-dim hover:text-accent">
                        "projects"
                    </A>
                    <A href=ROUTES.about attr:class="text-ink-dim hover:text-accent">
                        "about"
                    </A>
                    // Points at the search page rather than being an input, so
                    // it works before the `/search` route grows a form.
                    <A
                        href=ROUTES.search
                        attr:class="hidden gap-2 items-center px-3 py-1.5 rounded-sm border text-ink-dim border-rule bg-panel hover:text-accent sm:flex"
                    >
                        <SearchIcon />
                        <span class="text-xs">"search"</span>
                    </A>
                    <ThemeToggle />
                </nav>
            </div>
        </header>
    }
}
