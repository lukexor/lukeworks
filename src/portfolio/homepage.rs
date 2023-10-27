//! Portfolio homepage.

use crate::{
    portfolio::{
        about::About,
        blog::BlogSummary,
        constants::{homepage::intro, routes},
        contact::Contact,
        icons,
        projects::ProjectsSummary,
    },
    scroll_to_id,
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
            <div class="absolute text-center w-full left-0 bottom-4">
                {intro::ACTION}
                <br/>
                <a
                    class="icon-link text-3xl my-4"
                    class=icons::CHEVRON_DOWN
                    href=routes::HOME_ABOUT
                    on:click=|_| scroll_to_id(routes::HOME_ABOUT)
                    title=intro::EXPLORE
                ></a>
            </div>
        </div>
    }
}

/// Homepage.
#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <div class="h-screen bg-zinc-200 dark:bg-zinc-900 bg-intro-texture bg-fixed bg-no-repeat bg-cover bg-blend-screen dark:bg-blend-color-dodge">
            <div class="w-full max-w-prose h-full py-8 px-4 m-auto">
                <Intro/>
            </div>
        </div>
        <div>
            <div class="w-full max-w-prose h-full py-8 px-4 m-auto">
                <About/>
                <BlogSummary/>
                <ProjectsSummary/>
                <Contact/>
            </div>
        </div>
    }
}
