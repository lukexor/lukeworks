//! lukeworks.tech portfolio pages.

use crate::portfolio::{
    about::About, blog::Blog, contact::Contact, data::IconAttrs, header::Header,
    homepage::Homepage, post::Post, projects::Projects, tetanes_web::TetanesWeb,
};
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Body, Html, Link, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

pub mod about;
pub mod blog;
pub mod contact;
pub mod data;
pub mod errors;
pub mod header;
pub mod homepage;
pub mod post;
pub mod projects;
pub mod resume;
pub mod search;
pub mod tetanes_web;

/// Portfolio layout copy.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Layout {
    pub logo: &'static str,
    pub search_placeholder: &'static str,
    pub social_icons: [IconAttrs; 4],
}

#[component]
pub fn Portfolio() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" />
        <Title text="Lucas Petherbridge" />
        <Meta name="author" content="Lucas Petherbridge" />
        <Meta name="description" content="A blog and project portfolio by Lucas Petherbridge on programming, technology, and video game topics." />
        <Meta name="keywords" content="blog, programming, software, technology, video games, web design, web development" />
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lukeworks.css" />
        <Link rel="icon" href="/favicon.ico" />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <Link href="https://fonts.googleapis.com/css2?family=PT+Mono&family=Rubik&family=Yatra+One&display=swap" rel="stylesheet" />
        <Router>
            <Body class="mx-8 my-4 font-sans dark:bg-gray-700 dark:text-blue-400" />
            <Header />
            <main>
                <Routes>
                    <Route path="" view=Homepage />
                    <Route path="about" view=About />
                    <Route path="blog" view=Blog />
                    <Route path="contact" view=Contact />
                    <Route path="projects" view=Projects />
                    <Route path="tetanes-web" view=TetanesWeb />
                    <Route path=":post" view=Post />
                </Routes>
            </main>
        </Router>
    }
}
