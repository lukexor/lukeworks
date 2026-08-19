//! Shared post listing for `/blog` and `/projects`.
//!
//! Both pages are the same view over a different `kind`, so the data types, the
//! server function and the markup all live here rather than being written twice.

use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

/// Which listing to fetch.
///
/// Mirrors `crate::content::Kind`, which cannot be used directly: it is emitted
/// by `build.rs` without `Serialize`, and a server function argument has to
/// cross the network boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostKind {
    Blog,
    Project,
}

/// One row of a listing.
///
/// Deliberately excludes `body_html`. A listing of 26 posts would otherwise
/// serialize every rendered post into the page, which is the bulk of the site's
/// HTML — see the note on `fetch_post` about that tradeoff for a single post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    pub slug: String,
    pub title: String,
    pub category: Option<String>,
    pub published: Option<String>,
    pub reading_minutes: usize,
}

/// Fetch a listing from the compiled post table.
///
/// Goes through a server function for the same reason `fetch_post` does: the
/// `POSTS` table is `ssr`-only, so a component reading it directly would render
/// on the server and then blank out during hydration. leptos serializes the
/// resolved value into the page, so the first load costs no extra request.
#[server]
pub async fn list_posts(kind: PostKind) -> Result<Vec<PostSummary>, ServerFnError> {
    use crate::content::Kind;

    let kind = match kind {
        PostKind::Blog => Kind::Blog,
        PostKind::Project => Kind::Project,
    };

    // `published` already yields newest-first and filters drafts; build.rs did
    // the sorting, so nothing here parses a date to order the list.
    Ok(crate::content::published(kind)
        .map(|post| PostSummary {
            slug: post.slug.to_owned(),
            title: post.title.to_owned(),
            category: post.category.map(ToOwned::to_owned),
            published: post.published.map(ToOwned::to_owned),
            reading_minutes: post.reading_minutes,
        })
        .collect())
}

/// The date portion of an RFC 3339 timestamp.
///
/// Frontmatter dates are whole days whose time component means nothing, so the
/// string is truncated rather than parsed. No timezone handling, and nothing to
/// go wrong at runtime.
fn published_date(published: Option<&str>) -> Option<&str> {
    published.map(|value| value.split('T').next().unwrap_or(value))
}

/// Render the listing for one kind of post.
#[component]
pub fn PostList(kind: PostKind) -> impl IntoView {
    let posts = Resource::new(move || kind, list_posts);

    view! {
        <Suspense fallback=|| {
            view! { <p>"Loading…"</p> }
        }>
            {move || Suspend::new(async move {
                let posts = posts.await.unwrap_or_default();
                if posts.is_empty() {
                    return // Reached when a listing genuinely has no published posts,
                    // and when the server function errors — the reader needs
                    // the same thing either way.
                    view! { <p>"Nothing published here yet."</p> }
                        .into_any();
                }
                view! {
                    <ul class="flex flex-col gap-4">
                        {posts
                            .into_iter()
                            .map(|post| {
                                let href = format!("/{}", post.slug);
                                view! {
                                    <li>
                                        <A href=href attr:class="font-bold">
                                            {post.title.clone()}
                                        </A>
                                        <p class="text-sm">
                                            {published_date(post.published.as_deref())
                                                .map(ToOwned::to_owned)}
                                            {post
                                                .category
                                                .clone()
                                                .map(|category| format!(" · {category}"))}
                                            {format!(" · {} min read", post.reading_minutes)}
                                        </p>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                }
                    .into_any()
            })}
        </Suspense>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_date_drops_the_time_component() {
        assert_eq!(
            published_date(Some("2020-01-31T21:19:14Z")),
            Some("2020-01-31")
        );
        // A frontmatter date may be written without a time component.
        assert_eq!(published_date(Some("2020-01-31")), Some("2020-01-31"));
        assert_eq!(published_date(None), None);
    }
}
