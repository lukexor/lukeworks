//! Portfolio homepage.

use crate::portfolio::{
    about::About, blog::BlogSummary, constants::homepage::intro, contact::Contact,
    projects::ProjectsSummary,
};
use leptos::*;

/// Intro component.
#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="py-16 lg:pt-32">
            <h1 class="font-display text-5xl my-12 text-blue-500" inner_html=intro::TITLE/>
            <p class="font-display text-3xl my-12">
                {intro::SUBTITLE}
            </p>
            <p class="font-body text-md my-12" inner_html=intro::ABOUT/>
        </div>
    }
}

/// Homepage.
#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100dvh-6em)] max-w-prose m-auto">
            <Intro/>
        </div>
        <div class="bg-zinc-200 dark:bg-zinc-800">
            <div class="max-w-prose py-4 m-auto">
                <About/>
                <BlogSummary/>
                <ProjectsSummary/>
                <Contact/>
            </div>
        </div>
    }
}
