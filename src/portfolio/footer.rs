//! Portfolio Header components.

use crate::portfolio::{
    data::FOOTER,
    icons::{EmailIcon, GitHubIcon, LinkedInIcon, RssIcon},
};
use leptos::*;

/// Portfolio Footer component.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col lg:flex-row px-2 lg:px-12 py-4 justify-center items-center bg-gray-100 dark:bg-gray-700">
            <div class="px-2 lg:px-12" >
                <GitHubIcon />
                <LinkedInIcon />
                <RssIcon />
                <EmailIcon />
            </div>
            <div class="text-sm" inner_html={FOOTER.content} />
        </footer>
    }
}
