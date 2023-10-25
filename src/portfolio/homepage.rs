//! Portfolio homepage.

use crate::{
    portfolio::{
        about::About,
        blog::BlogSummary,
        contact::Contact,
        data::{INTRO, LAYOUT},
        projects::ProjectsSummary,
    },
    scroll_to_id,
};
use leptos::*;

/// Intro component.
#[component]
pub fn Intro() -> impl IntoView {
    let explore_icon = LAYOUT.icons.explore;
    view! {
        <div class="py-16 lg:pt-32">
            <h1 class="font-display text-5xl my-12 text-blue-500">
                {INTRO.title1} " " <span class="text-red-400">{INTRO.title2}</span> "."
            </h1>
            <p class="font-display text-3xl my-12">{INTRO.subtitle}</p>
            <p class="font-body text-md my-12">{INTRO.about}</p>
            <div class="absolute text-center w-full left-0 bottom-4">
                {INTRO.action} <br/>
                <a
                    class="icon-link text-3xl my-4"
                    class=explore_icon.icon
                    href=explore_icon.href
                    on:click=|_| scroll_to_id(explore_icon.href)
                    title=explore_icon.title
                ></a>
            </div>
        </div>
    }
}

/// Homepage.
#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <div class="bg-gray-200 dark:bg-blue-700 bg-intro-texture bg-fixed bg-no-repeat bg-cover bg-blend-screen dark:bg-blend-color-dodge">
            <div class="min-h-[calc(100dvh-3.4rem)] backdrop-blur-sm">
                <div class="w-full max-w-prose h-full py-8 px-4 m-auto">
                    <Intro/>
                </div>
            </div>
        </div>
        <div class="bg-gray-200 dark:bg-blue-700">
            <div class="w-full max-w-prose h-full py-8 px-4 m-auto">
                <About/>
                <BlogSummary/>
                <ProjectsSummary/>
                <Contact/>
            </div>
        </div>
    }
}
