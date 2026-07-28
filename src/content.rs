//! Post content, compiled from `content/posts/*.md` by `build.rs`.
//!
//! The generated table holds every post's metadata plus its body already
//! rendered to HTML. Bodies are compiled into the `ssr` build only; under
//! `hydrate` `POSTS` is an empty slice, which keeps the post content out of the
//! WASM bundle. That is safe because in islands mode `#[component]` bodies are
//! server-rendered and never execute in the browser — an island that needs post
//! data must reach for it through a server function.

include!(concat!(env!("OUT_DIR"), "/content.rs"));

/// Look up a single post by its slug (the markdown filename without extension).
#[must_use]
pub fn find(slug: &str) -> Option<&'static Post> {
    POSTS.iter().find(|post| post.slug == slug)
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

// Only meaningful against the generated table, which is compiled into the `ssr`
// build. Run with `cargo nextest run --features ssr`.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn every_post_compiled() {
        // Guards against build.rs silently emitting nothing, which would
        // otherwise look like a site with no content rather than a build error.
        assert_eq!(POSTS.len(), 27, "expected every content/posts/*.md");
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
        // mindyou has a body but never had a metadata entry, so it was
        // unreachable on the old site and stays unlisted here.
        let draft = find("mindyou").expect("mindyou should still compile");
        assert!(draft.draft);
        assert!(published(Kind::Project).all(|post| post.slug != "mindyou"));
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
