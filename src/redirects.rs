//! Permanent redirects carried over from the Next.js site.
//!
//! The table is compiled from `content/redirects.toml` by `build.rs`, so the
//! only thing that happens per request is a walk over a static slice. See that
//! file for the rules themselves and for what was dropped during the port.
//!
//! These are the URLs the old site published for years, so they carry real
//! search traffic. They also fail silently — a broken rule looks like an
//! ordinary 404 — which is why the table is exercised directly by tests below
//! rather than only through the middleware.

include!(concat!(env!("OUT_DIR"), "/redirects.rs"));

/// Resolve `path` against the redirect table.
///
/// Returns the destination for the first matching rule, or `None` when nothing
/// matches and the request should be routed normally.
///
/// Matching is order-sensitive by design: the specific `/articles/<date>/<slug>`
/// rules are declared before the generic one and must win.
#[must_use]
pub fn resolve(path: &str) -> Option<String> {
    // Trailing slashes are trimmed rather than treated as a distinct path.
    // Next.js normalised these away before its own redirect table ran, so
    // `/articles/2020/01/foo/` reached the same rule as `/articles/2020/01/foo`.
    let segments: Vec<&str> = path.trim_matches('/').split('/').collect();
    // `"/"` splits to `[""]`, which would otherwise look like a one-segment path
    // and match a single-`Param` rule.
    let segments: &[&str] = if segments == [""] { &[] } else { &segments };

    REDIRECTS
        .iter()
        .find_map(|rule| match_rule(rule, segments).map(|captures| expand(rule.to, &captures)))
}

/// Match one rule, returning its captured `(name, value)` pairs on success.
fn match_rule<'a>(rule: &Redirect, segments: &[&'a str]) -> Option<Vec<(&'static str, &'a str)>> {
    // Every pattern segment consumes exactly one path segment, so a length
    // mismatch can never match — no need to walk.
    if rule.from.len() != segments.len() {
        return None;
    }

    let mut captures = Vec::new();
    for (pattern, actual) in rule.from.iter().zip(segments) {
        match pattern {
            Segment::Literal(text) if text == actual => {}
            Segment::Literal(_) => return None,
            // An empty segment (`//` in the path) is not a value worth
            // capturing; refusing it keeps `/articles//01/foo` from
            // redirecting to `/foo` with a hole in the middle.
            Segment::Param(_) if actual.is_empty() => return None,
            Segment::Param(name) => captures.push((*name, *actual)),
        }
    }
    Some(captures)
}

/// Substitute captured params into a destination pattern.
fn expand(to: &str, captures: &[(&str, &str)]) -> String {
    if captures.is_empty() || !to.contains(':') {
        return to.to_owned();
    }

    let mut out = String::with_capacity(to.len());
    for (index, part) in to.split('/').enumerate() {
        if index > 0 {
            out.push('/');
        }
        match part.strip_prefix(':') {
            // build.rs already rejected a `:name` the `from` pattern cannot
            // capture, so the lookup below always hits.
            Some(name) => out.push_str(
                captures
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map_or(part, |(_, value)| value),
            ),
            None => out.push_str(part),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_is_not_empty() {
        // Guards against build.rs emitting nothing, which would look exactly
        // like "no rule matched" at every call site.
        assert!(!REDIRECTS.is_empty());
    }

    #[test]
    fn specific_article_rules_beat_the_generic_one() {
        // Why declaration order matters. If the generic
        // `/articles/:year/:month/:title` rule were tried first, these would
        // resolve to `/rustynes_part_1` and `/rustynes_part_2`, which are not
        // posts.
        assert_eq!(
            resolve("/articles/2019/09/rustynes_part_1").as_deref(),
            Some("/tetanes-part-1")
        );
        assert_eq!(
            resolve("/articles/2020/01/rustynes_part_2").as_deref(),
            Some("/tetanes-part-2")
        );
        assert_eq!(
            resolve("/projects/2019/08/rustynes").as_deref(),
            Some("/tetanes")
        );
    }

    #[test]
    fn the_generic_article_rule_captures_the_slug() {
        assert_eq!(
            resolve("/articles/2021/06/software-malaise").as_deref(),
            Some("/software-malaise")
        );
        assert_eq!(
            resolve("/articles/1999/12/anything-at-all").as_deref(),
            Some("/anything-at-all")
        );
    }

    #[test]
    fn listing_and_feed_rules_resolve() {
        assert_eq!(resolve("/articles").as_deref(), Some("/blog"));
        assert_eq!(resolve("/feed").as_deref(), Some("/rss"));
    }

    #[test]
    fn trailing_slashes_are_normalised() {
        assert_eq!(resolve("/articles/").as_deref(), Some("/blog"));
        assert_eq!(
            resolve("/articles/2021/06/software-malaise/").as_deref(),
            Some("/software-malaise")
        );
    }

    #[test]
    fn live_routes_are_left_alone() {
        // These are real pages now. A rule shadowing one would make it
        // unreachable, which is exactly what the dropped anchor rules did.
        for path in ["/", "/about", "/blog", "/projects", "/tetanes-part-1"] {
            assert_eq!(resolve(path), None, "{path} should not redirect");
        }
    }

    #[test]
    fn dropped_rules_stay_dropped() {
        // `/category/*` and `/tag/*` were never real destinations; these must
        // 404 rather than redirect somewhere equally missing.
        assert_eq!(resolve("/articles/category/programming"), None);
        assert_eq!(resolve("/blog/tag/rust"), None);
        // The anchor rules that would have shadowed live pages.
        assert_eq!(resolve("/contact"), None);
    }

    #[test]
    fn segment_counts_must_match_exactly() {
        // A `:param` matches one segment, never several.
        assert_eq!(resolve("/articles/2020/01"), None);
        assert_eq!(resolve("/articles/2020/01/a/b"), None);
    }

    #[test]
    fn empty_segments_do_not_capture() {
        // Would otherwise redirect to "/" with the slug silently dropped.
        assert_eq!(resolve("/articles/2020/01//"), None);
    }

    #[test]
    fn every_destination_is_a_resolved_path() {
        // A rule whose `to` still contains a `:` would emit a Location header
        // with a literal ":name" in it.
        for rule in REDIRECTS {
            let params: Vec<_> = rule
                .from
                .iter()
                .filter_map(|segment| match segment {
                    Segment::Param(name) => Some(*name),
                    Segment::Literal(_) => None,
                })
                .collect();
            let sample: Vec<&str> = rule
                .from
                .iter()
                .map(|segment| match segment {
                    Segment::Literal(text) => *text,
                    Segment::Param(_) => "sample",
                })
                .collect();
            let path = format!("/{}", sample.join("/"));
            let resolved = resolve(&path)
                .unwrap_or_else(|| panic!("{path} matched no rule, but was built from one"));
            assert!(
                !resolved.contains(':'),
                "{path} -> {resolved} still contains an unsubstituted param (captures: {params:?})"
            );
        }
    }

    #[test]
    fn redirects_do_not_point_at_other_redirects() {
        // A chain adds a round trip, and search engines stop following one
        // after a few hops.
        for rule in REDIRECTS {
            let sample: Vec<&str> = rule
                .from
                .iter()
                .map(|segment| match segment {
                    Segment::Literal(text) => *text,
                    Segment::Param(_) => "sample",
                })
                .collect();
            let destination = resolve(&format!("/{}", sample.join("/"))).unwrap();
            assert_eq!(
                resolve(&destination),
                None,
                "{destination} is itself redirected"
            );
        }
    }
}
