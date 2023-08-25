//! lukeworks.tech

use crate::lukeworks::{
    about::About, blog::Blog, contact::Contact, editor::Editor, homepage::Homepage, post::Post,
    projects::Projects, tetanes_web::TetanesWeb,
};
use leptos::{component, view, IntoView, Scope};
use leptos_meta::{provide_meta_context, Body, Html, Link, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

// include_str!("../data/");

pub mod about;
pub mod blog;
pub mod contact;
pub mod editor;
pub mod errors;
pub mod homepage;
pub mod post;
pub mod projects;
pub mod resume;
pub mod search;
pub mod tetanes_web;

#[component]
pub fn Logo(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="text-3xl dark:text-blue-500 font-monospace font-semibold">
            <span class="dark:text-red-400">"❱"</span>
            "L"
        </div>
    }
}

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="m-5 dark:text-blue-500">
            <ul>
                <li>Home</li>
            </ul>
        </div>
    }
}

#[component]
pub fn LukeWorks(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
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
            <Logo />
            <Nav />
            <main>
                <Routes>
                    <Route path="" view=Homepage />
                    <Route path="about" view=About />
                    <Route path="blog" view=Blog />
                    <Route path="contact" view=Contact />
                    <Route path="editor" view=Editor />
                    <Route path="projects" view=Projects />
                    <Route path="tetanes-web" view=TetanesWeb />
                    <Route path=":post" view=Post />
                </Routes>
            </main>
        </Router>
    }
}
