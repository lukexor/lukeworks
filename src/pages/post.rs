//! A single blog or project post.

use crate::pages::not_found::NotFound;
use leptos::{either::EitherOf3, prelude::*};
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
/// The compiled `POSTS` table exists only in the `ssr` build, which keeps
/// ~124KB of rendered HTML out of the WASM bundle. A server function reaches it
/// on the client's behalf, so the client renders posts anyway. On first load
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
                    Ok(Some(post)) => {
                        EitherOf3::A(
                            view! {
                                <Title text=post.title.clone() />
                                <article>
                                    <h1>{post.title.clone()}</h1>
                                    // Pre-rendered at build time from markdown we control.
                                    <div inner_html=post.body_html.clone()></div>
                                </article>
                            },
                        )
                    }
                    Ok(None) => EitherOf3::B(view! { <NotFound /> }),
                    Err(_) => {
                        EitherOf3::C(
                            // Kept apart from the `None` arm: the post may well exist and
                            // the fetch just failed, so telling the reader the URL is
                            // wrong sends them away from a page that would load on a
                            // retry.
                            view! {
                                <Title text="Couldn't load this post" />
                                <div class="flex flex-col items-center w-full">
                                    <h1 class="my-4 text-3xl">"Couldn't load this post."</h1>
                                    <p>
                                        "Something went wrong on our end. Try reloading the page."
                                    </p>
                                </div>
                            },
                        )
                    }
                }
            })}
        </Suspense>
    }
}
