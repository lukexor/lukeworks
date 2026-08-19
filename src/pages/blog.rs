use crate::components::post_list::{PostKind, PostList};
use leptos::prelude::*;
use leptos_meta::Title;

/// Blog entries.
#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Title text="Blog" />
        <h1 class="my-4 text-3xl">"Blog"</h1>
        <PostList kind=PostKind::Blog />
    }
}
