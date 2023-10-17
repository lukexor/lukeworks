//! lukeworks.tech portfolio pages.

use crate::portfolio::{
    blog::Blog, data::ROUTES, footer::Footer, header::Header, homepage::Homepage, post::Post,
    tetanes_web::TetanesWeb,
};
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Body, Html, Link, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

#[cfg(feature = "ssr")]
pub mod rss;

pub mod about;
pub mod blog;
pub mod contact;
pub mod data;
pub mod errors;
pub mod footer;
pub mod header;
pub mod homepage;
pub mod icons;
pub mod post;
pub mod projects;
pub mod resume;
pub mod search;
pub mod tetanes_web;

/// LukeWorks portfolio.
#[component]
pub fn Portfolio() -> impl IntoView {
    provide_meta_context();
    let prefers_dark = header::initial_prefers_dark();
    let color_scheme = move || {
        if prefers_dark {
            "dark"
        } else {
            "light"
        }
    };

    view! {
        <Html lang="en" class={color_scheme()} />
        <Meta http_equiv="X-UA-Compatible" content="ie=edge" />

        <Title text="Lucas Petherbridge" />

        <Meta name="author" content="Lucas Petherbridge" />
        <Meta name="description" content="A blog and project portfolio by Lucas Petherbridge on programming, technology, and video game topics." />
        <Meta name="keywords" content="blog, programming, software, technology, video games, web design, web development" />

        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lukeworks.css" />
        <Link rel="apple-touch-icon" sizes="180x180" href="/icons/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/icons/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/icons/favicon-16x16.png" />
        <Link rel="manifest" href="/site.webmanifest" />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=PT+Mono&family=Rubik&family=Yatra+One&display=swap" />

        <Router>
            <Body class="font-body bg-gray-200 dark:bg-gray-800 text-blue-400" />
            <Header />
            <main>
                <Routes>
                    <Route path={ROUTES.home.path} view=Homepage />
                    <Route path={ROUTES.tetanes_web.path} view=TetanesWeb />
                    <Route path={ROUTES.post.path} view=Post />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
