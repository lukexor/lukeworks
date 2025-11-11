use crate::{
    hooks::use_theme::{Theme, use_theme},
    pages::{home::Home, not_found::NotFound, post::Post},
};
use leptos::{either::Either, prelude::*};
use leptos_meta::{
    HashedStylesheet, Link, Meta, MetaTags, Stylesheet, Title, provide_meta_context,
};
use leptos_router::{
    ParamSegment, StaticSegment,
    components::{FlatRoutes, Route, Router, RoutingProgress},
};

/// Support email.
pub const SUPPORT_EMAIL: &str = "me@lukeworks.tech";
/// Application routes.
pub const ROUTES: AppRoutes = AppRoutes { home: "/" };

/// Type for application routes.
#[derive(Debug)]
#[must_use]
pub struct AppRoutes {
    pub home: &'static str,
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

                <Link rel="icon" href="/favicon.ico" />
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

    let Theme {
        prefers_dark,
        toggle_prefers_dark,
    } = use_theme();
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <noscript>
            "This page contains WebAssembly and Javascript content, please enable
            Javascript in your browser."
        </noscript>

        <Title formatter=move |text| format!("{text} — Lucas Petherbridge | Software Engineer") />
        // This can't really be reactive, so use_theme uses an Effect to synchronize it
        <Meta name="color-scheme" content="" />

        <Router set_is_routing>
            // Ensure progress bar is above header
            <div class="z-20 [&_progress]:h-[2px] [&_progress]:w-full [&_progress]:absolute [&_progress]:text-brand-fg2 [&_progress::-moz-progress-bar]:bg-brand-fg2 [&_progress::-webkit-progress-value]:bg-brand-fg2">
                <RoutingProgress is_routing />
            </div>
            <Header />
            <main>
                <FlatRoutes transition=true fallback=NotFound>
                    <Route path=StaticSegment("") view=Home />
                    <Route path=StaticSegment("/about") view=About />
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
            <a href=format!("email:{SUPPORT_EMAIL}")>"bug report"</a>
            "."
        </p>
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
