use crate::pages::not_found::NotFound;
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, Debug, Clone, PartialEq)]
#[must_use]
struct PostParams {
    post: Option<String>,
}

/// Post entry.
#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = Signal::derive(move || {
        params.with(|params| {
            params
                .as_ref()
                .ok()
                .and_then(|p| p.post.as_deref().and_then(find_post))
        })
    });

    match post.get_untracked() {
        Some(post) => Either::Left(view! {
            <Title text="Post" />
            <div>{post.slug}</div>
        }),
        None => Either::Right(view! { <NotFound /> }),
    }
}

#[derive(Debug, Clone)]
#[must_use]
struct Post {
    slug: String,
    title: String,
}

fn find_post(post: &str) -> Option<Post> {
    const POSTS: &str = include_str!("../../content/blog_posts.toml");

    if post == "tetanes" {
        Some(Post {
            slug: post.to_string(),
            title: String::new(),
        })
    } else {
        None
    }
}
