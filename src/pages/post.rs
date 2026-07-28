//! A single blog or project post.

use crate::pages::not_found::NotFound;
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::{hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

#[derive(Params, Debug, Clone, PartialEq)]
#[must_use]
struct PostParams {
    post: Option<String>,
}

/// A post as sent to the browser.
///
/// Owned rather than `&'static str` because this crosses the network boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostView {
    pub slug: String,
    pub title: String,
    pub body_html: String,
    pub reading_minutes: usize,
    pub published: Option<String>,
}

/// Look up a post on the server.
///
/// The compiled `POSTS` table exists only in the `ssr` build, deliberately —
/// keeping ~124KB of rendered HTML out of the WASM bundle. Reaching it through
/// a server function is what lets the client render posts anyway. On first load
/// the resolved value is serialized into the page, so hydration does not make a
/// second request; only a client-side navigation to a different post does.
#[server]
pub async fn fetch_post(slug: String) -> Result<Option<PostView>, ServerFnError> {
    Ok(crate::content::find(&slug).map(|post| PostView {
        slug: post.slug.to_owned(),
        title: post.title.to_owned(),
        body_html: post.body_html.to_owned(),
        reading_minutes: post.reading_minutes,
        published: post.published.map(ToOwned::to_owned),
    }))
}

/// Post entry.
#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let slug =
        move || params.with(|params| params.as_ref().ok().and_then(|params| params.post.clone()));

    let post = Resource::new(slug, |slug| async move {
        match slug {
            Some(slug) => fetch_post(slug).await,
            None => Ok(None),
        }
    });

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(Some(post)) => Either::Left(view! {
                        <Title text=post.title.clone() />
                        <article>
                            <h1>{post.title.clone()}</h1>
                            // Pre-rendered at build time from markdown we control.
                            <div inner_html=post.body_html.clone()></div>
                        </article>
                    }),
                    // A lookup failure and a missing slug are the same thing to
                    // a reader: the URL doesn't name a post.
                    Ok(None) | Err(_) => Either::Right(view! { <NotFound /> }),
                }
            })}
        </Suspense>
    }
}
