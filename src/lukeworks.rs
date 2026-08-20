use crate::{
    components::{footer::Footer, icons::SearchIcon, theme_toggle::ThemeToggle},
    hooks::use_theme,
    pages::{
        about::About, blog::Blog, home::Home, not_found::NotFound, post::Post, projects::Projects,
        search::Search,
    },
};
use leptos::{either::EitherOf3, prelude::*};
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

/// The whole compiled stylesheet, read from the site directory on first render.
///
/// `None` if the file is not where the options say it is, which leaves
/// [`RootStylesheet`] on the `<link>` it would otherwise have rendered.
///
/// Held in a `OnceLock` because `shell` runs per request and the file does not
/// change under a running server.
///
/// Always `None` outside a release server build: `cargo leptos watch` writes
/// the file after the server is already up, and the WASM target has no disk.
#[cfg(all(feature = "ssr", not(debug_assertions)))]
fn compiled_css(options: &LeptosOptions) -> Option<&'static str> {
    use std::sync::OnceLock;

    static CSS: OnceLock<Option<String>> = OnceLock::new();

    CSS.get_or_init(|| {
        // The same two spellings `HashedStylesheet` picks between, and the same
        // place it looks for the hash: beside the binary, not under the site
        // root. Deriving the name here rather than reading the directory keeps
        // a stale `lukeworks.<old-hash>.css` from being served.
        let mut name = options.output_name.to_string();
        if options.hash_files {
            let hash_path = std::env::current_exe()
                .ok()?
                .parent()?
                .join(options.hash_file.as_ref());
            let hashes = std::fs::read_to_string(hash_path).ok()?;
            let hash = hashes
                .lines()
                .filter_map(|line| line.trim().split_once(':'))
                .find(|&(file, _)| file == "css")?
                .1;
            name.push('.');
            name.push_str(hash.trim());
        }
        let path = std::path::Path::new(options.site_root.as_ref())
            .join(options.site_pkg_dir.as_ref())
            .join(format!("{name}.css"));
        std::fs::read_to_string(path).ok()
    })
    .as_deref()
}

#[cfg(not(all(feature = "ssr", not(debug_assertions))))]
fn compiled_css(_options: &LeptosOptions) -> Option<&'static str> {
    None
}

/// Puts the stylesheet in the document head.
///
/// Release builds inline it. A `<link>` is the page's only render-blocking
/// request, and Lighthouse charges it 150ms on a throttled mobile connection.
/// The file is ~6KB over the wire, which every page now carries in exchange
/// for that round trip.
///
/// Dev builds keep the `<link>`, so `cargo leptos watch` can still hot-swap the
/// stylesheet by its href.
#[component]
fn RootStylesheet(options: LeptosOptions) -> impl IntoView {
    if let Some(css) = compiled_css(&options) {
        // `inner_html` rather than a text child: a text child is escaped, and
        // Tailwind's `>` combinators would arrive as `&gt;`.
        EitherOf3::A(view! { <style id="style" inner_html=css></style> })
    } else if options.hash_files {
        EitherOf3::B(view! { <HashedStylesheet id="style" options /> })
    } else {
        let href = format!(
            "{pkg_path}/{css_name}.css",
            pkg_path = &options.site_pkg_dir,
            css_name = &options.output_name
        );
        EitherOf3::C(view! { <Stylesheet id="style" href /> })
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

                // A font is undiscoverable until the stylesheet naming it has
                // parsed, so both files sat behind the CSS on the critical
                // path: 113ms in Lighthouse. Preloading starts them alongside
                // it.
                //
                // Plain `<link>` rather than `<Link>`: leptos_meta's typed
                // props drop `as` and `crossorigin`, and a preload missing
                // either is inert. `crossorigin` is required even same-origin,
                // because a font fetches in CORS mode.
                <link
                    rel="preload"
                    href="/fonts/jetbrains-mono.woff2"
                    r#as="font"
                    r#type="font/woff2"
                    crossorigin="anonymous"
                />
                <link
                    rel="preload"
                    href="/fonts/ibm-plex-sans.woff2"
                    r#as="font"
                    r#type="font/woff2"
                    crossorigin="anonymous"
                />

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
                    // Every route holding a `Suspense` needs `SsrMode::Async`.
                    // Under the default out-of-order mode the suspended part
                    // streams into a trailing <template> for JS to swap in, so
                    // <main> ships without it and a crawler or a reader without
                    // JS sees an empty page. Async holds the response until the
                    // resource resolves, which is microseconds against an
                    // in-memory table.
                    <Route path=StaticSegment("") view=Home ssr=SsrMode::Async />
                    <Route path=StaticSegment("/about") view=About />
                    <Route path=StaticSegment("/search") view=Search />
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
                    // Narrow screens get it in the footer instead, where the
                    // row has space the header's four nav links do not.
                    <span class="hidden sm:inline">
                        <ThemeToggle />
                    </span>
                </nav>
            </div>
        </header>
    }
}
