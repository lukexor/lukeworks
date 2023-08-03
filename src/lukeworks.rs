//! lukeworks.tech

use crate::{
    about::{About, AboutProps},
    blog::{Blog, BlogProps},
    contact::{Contact, ContactProps},
    editor::{Editor, EditorProps},
    homepage::{Homepage, HomepageProps},
    post::{Post, PostProps},
    projects::{Projects, ProjectsProps},
    resume::{Resume, ResumeProps},
    tetanes_web::{TetaNESWeb, TetaNESWebProps},
};
use leptos::{component, view, IntoView, Scope};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        "Nav"
    }
}

#[component]
pub fn LukeWorks(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <main>
                <Routes>
                    <Route path="/resume" view=|cx| view! { cx, <Resume /> } />
                    <Route path="/" view=|cx| view! { cx, <Nav/> }>
                        <Route path="/" view=|cx| view! { cx, <Homepage /> } />
                        <Route path="/about" view=|cx| view! { cx, <About /> } />
                        <Route path="/projects" view=|cx| view! { cx, <Projects /> } />
                        <Route path="/blog" view=|cx| view! { cx, <Blog /> } />
                        <Route path="/contact" view=|cx| view! { cx, <Contact /> } />
                        <Route path="/:post" view=|cx| view! { cx, <Post /> } />
                        <Route path="/tetanes-web" view=|cx| view! { cx, <TetaNESWeb /> } />
                        <Route path="/editor" view=|cx| view! { cx, <Editor /> } />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
