/// Portfolio posts.
use crate::portfolio::errors::NotFound;
use leptos::{component, view, IntoView, SignalWith};
use leptos_router::use_params_map;

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let post = params.with(|param| param.get("post").cloned().unwrap_or_default());
    if post.is_empty() {
        view! {  <NotFound /> }
    } else {
        view! {
            "Post"
        }
        .into_view()
    }
}
