//! Post content, compiled from `content/posts/*.md` by `build.rs`.
//!
//! The generated table holds every post's metadata plus its body already
//! rendered to HTML. Bodies are compiled into the `ssr` build only; under
//! `hydrate` `POSTS` is an empty slice, which keeps ~124KB of rendered HTML out
//! of the WASM bundle.
//!
//! **Because of that, this module is server-only in practice.** Component
//! bodies re-run in the browser during hydration, so a component that reads
//! `POSTS` directly would find it empty there and render as though the content
//! did not exist. Reach it through a server function instead — see
//! [`crate::pages::post::fetch_post`], whose resolved value leptos serializes
//! into the page so hydration costs no extra request.

include!(concat!(env!("OUT_DIR"), "/content.rs"));

/// Look up a published post by its slug (the markdown filename without extension).
///
/// Applies the same draft and `published` filter as [`published`]. Slugs are
/// guessable, so a draft reachable at its own URL is a draft anyone can read.
#[must_use]
pub fn find(slug: &str) -> Option<&'static Post> {
    POSTS
        .iter()
        .find(|post| post.slug == slug && !post.draft && post.published.is_some())
}

/// Published posts of the given kind, newest first.
///
/// `build.rs` emits `POSTS` already ordered by publication date, so this only
/// filters — no sorting or date parsing happens at runtime.
pub fn published(kind: Kind) -> impl Iterator<Item = &'static Post> {
    POSTS
        .iter()
        .filter(move |post| post.kind == kind && !post.draft && post.published.is_some())
}

/// Every published post in `series`, in reading order.
///
/// A post with no `part` is the series introduction and comes first. The rest
/// follow by part number. `build.rs` rejects a duplicate part, so the order is
/// total. Returns an empty vec for a series nobody belongs to.
#[must_use]
pub fn series(name: &str) -> Vec<&'static Post> {
    let mut entries: Vec<_> = POSTS
        .iter()
        .filter(|post| post.series == Some(name) && !post.draft && post.published.is_some())
        .collect();
    entries.sort_by_key(|post| post.part.unwrap_or(0));
    entries
}

// Only meaningful against the generated table, which is compiled into the `ssr`
// build. Run with `cargo nextest run --features ssr`.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn every_post_compiled() {
        // Guards against build.rs silently emitting nothing, which would
        // otherwise look like a site with no content rather than a build error.
        // A lower bound rather than an exact count, so authoring a post is not
        // also a test failure.
        assert!(!POSTS.is_empty(), "expected every content/posts/*.md");
    }

    #[test]
    fn slugs_are_unique() {
        // Slugs are URLs, and `find` returns the first match, so a duplicate
        // would silently shadow a post.
        let mut seen = std::collections::HashSet::new();
        for post in POSTS {
            assert!(seen.insert(post.slug), "duplicate slug: {}", post.slug);
        }
    }

    #[test]
    fn every_post_has_a_body() {
        for post in POSTS {
            assert!(!post.body_html.is_empty(), "{} rendered empty", post.slug);
            assert!(
                post.reading_minutes >= 1,
                "{} has no reading time",
                post.slug
            );
        }
    }

    #[test]
    fn find_resolves_known_slug() {
        let post = find("tetanes-part-2").expect("tetanes-part-2 should exist");
        assert_eq!(post.kind, Kind::Blog);
        assert!(post.body_html.contains("<p>"));
        assert!(find("no-such-post").is_none());
    }

    #[test]
    fn listings_are_newest_first() {
        // build.rs sorts, so nothing at runtime re-derives this ordering.
        for kind in [Kind::Blog, Kind::Project] {
            let dates: Vec<_> = published(kind).filter_map(|post| post.published).collect();
            assert!(!dates.is_empty(), "{kind:?} listing is empty");
            let mut sorted = dates.clone();
            sorted.sort_unstable_by(|a, b| b.cmp(a));
            assert_eq!(dates, sorted, "{kind:?} listing is out of order");
        }
    }

    #[test]
    fn drafts_are_excluded_from_listings() {
        // No draft posts exist right now, so this asserts the rule rather than
        // a specific post: anything unpublished or flagged stays out of listings.
        for kind in [Kind::Blog, Kind::Project] {
            assert!(
                published(kind).all(|post| !post.draft && post.published.is_some()),
                "{kind:?} listing includes a draft"
            );
        }
    }

    #[test]
    fn find_applies_the_same_draft_rule_as_listings() {
        // A slug is guessable, so a draft hidden from `/blog` but readable at
        // `/its-slug` is not hidden at all.
        for post in POSTS {
            if post.draft || post.published.is_none() {
                assert!(
                    find(post.slug).is_none(),
                    "{} is unpublished but reachable by slug",
                    post.slug
                );
            }
        }
    }

    #[test]
    fn a_series_reads_intro_first_then_parts_in_order() {
        let entries = series("Lost and Found");
        assert_eq!(
            entries.iter().map(|p| p.slug).collect::<Vec<_>>(),
            [
                "lost-and-found-series",
                "lost-and-found-part-1",
                "lost-and-found-part-2",
                "lost-and-found-part-3",
                "lost-and-found-part-4",
                "lost-and-found-part-5",
            ]
        );
        // Publication order happens to agree here. `part` orders the list, so a
        // backfilled post still lands in the right place.
        assert_eq!(entries[0].part, None);
        assert_eq!(entries[5].part, Some(5));
    }

    #[test]
    fn an_unknown_series_is_empty_rather_than_a_panic() {
        assert!(series("No Such Series").is_empty());
    }

    #[test]
    fn every_part_belongs_to_a_series() {
        // `build.rs` rejects this, so a failure here means the check regressed.
        for post in POSTS {
            assert!(
                post.part.is_none() || post.series.is_some(),
                "{} has a part but no series",
                post.slug
            );
        }
    }

    #[test]
    fn no_post_hotlinks_production() {
        // These images and sketch routes are served locally; absolute URLs to
        // the live site would bypass them and break offline/dev rendering.
        for post in POSTS {
            assert!(
                !post.body_html.contains("https://lukeworks.tech/images/")
                    && !post.body_html.contains("https://lukeworks.tech/sketch/"),
                "{} hotlinks the production site",
                post.slug
            );
        }
    }
}
