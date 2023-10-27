//! Portfolio Header components.

use crate::portfolio::{
    constants::{layout, meta},
    icons::{EmailIcon, GitHubIcon, LinkedInIcon, RssIcon},
};
use chrono::{Datelike, Utc};
use leptos::*;

/// Portfolio Footer component.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col lg:flex-row px-2 lg:px-12 py-4 justify-center items-center bg-zinc-300 dark:bg-zinc-900">
            <div class="px-2 lg:px-12" >
                <GitHubIcon />
                <LinkedInIcon />
                <RssIcon />
                <EmailIcon />
            </div>
            <div class="text-sm">
                {layout::footer(Utc::now().year(), meta::AUTHOR)}
            </div>
        </footer>
    }
}
