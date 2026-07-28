//! A single blog or project post.

use crate::{content, pages::not_found::NotFound};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, Debug, Clone, PartialEq)]
#[must_use]
struct PostParams {
    post: Option<String>,
}

/// Post entry.
///
/// Bodies were rendered to HTML by `build.rs`, so this is a lookup in a static
/// table — no I/O, no markdown parsing, nothing to await.
#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = move || {
        params.with(|params| {
            params
                .as_ref()
                .ok()
                .and_then(|params| params.post.as_deref())
                .and_then(content::find)
        })
    };

    move || match post() {
        Some(post) => Either::Left(view! {
            <Title text=post.title />
            <article>
                <h1>{post.title}</h1>
                // Pre-rendered at build time from markdown we control.
                <div inner_html=post.body_html></div>
            </article>
        }),
        None => Either::Right(view! { <NotFound /> }),
    }
}
