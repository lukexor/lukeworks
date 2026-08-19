use crate::components::post_list::{PostKind, PostList};
use leptos::prelude::*;
use leptos_meta::Title;

/// Project entries.
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="Projects" />
        <h1 class="my-4 text-3xl">"Projects"</h1>
        <PostList kind=PostKind::Project />
    }
}
