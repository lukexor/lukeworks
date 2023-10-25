//! lukeworks.tech portfolio pages.

use crate::{
    i18n::*,
    portfolio::{
        data::ROUTES, footer::Footer, header::Header, homepage::Homepage, post::Post,
        tetanes_web::TetanesWeb,
    },
};
use leptos::*;
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
    provide_i18n_context();

    let i18n = use_i18n();

    let prefers_dark = header::initial_prefers_dark();
    let dark_mode = move || if prefers_dark { "dark" } else { "" };

    view! {
        <Html lang="en" class=move || format!("!scroll-smooth scroll-pt-14 {}", dark_mode())/>
        <Meta http_equiv="X-UA-Compatible" content="ie=edge"/>

        <Title text=t!(i18n, meta_title)/>

        <Meta name="author" content=t!(i18n, meta_author)/>
        <Meta name="description" content=t!(i18n, meta_desc)/>
        <Meta name="keywords" content=t!(i18n, meta_keywords)/>

        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lukeworks.css"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/icons/apple-touch-icon.png"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/icons/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/icons/favicon-16x16.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=PT+Mono&family=Rubik&family=Yatra+One&display=swap"
        />

        <Router>
            <Body class="font-body bg-gray-200 dark:bg-blue-700 text-blue-400"/>
            <Header/>
            <main>
                <Routes>
                    <Route path=ROUTES.home.path view=Homepage/>
                    <Route path=ROUTES.tetanes_web.path view=TetanesWeb/>
                    <Route path=ROUTES.post.path view=Post/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
