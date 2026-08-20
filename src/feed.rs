//! RSS 2.0 feed over the published blog posts.
//!
//! Served by a plain Axum handler rather than a Leptos route: the response is
//! XML, not a document, so there is no shell to render and nothing to hydrate.

use crate::content::{self, Kind};
use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

/// Canonical origin, used to absolutise the links a feed reader follows.
pub const SITE_URL: &str = "https://lukeworks.tech";
/// Feed title.
const TITLE: &str = "Lucas Petherbridge";
/// Feed description.
const DESCRIPTION: &str = "A blog and project portfolio by Lucas Petherbridge on programming, technology, and video games.";

/// Escape the five characters that cannot appear as text in XML.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Rewrite the root-relative links in a post body to absolute ones.
///
/// `no_post_hotlinks_production` requires post bodies to link with a leading
/// `/`. RSS gives the HTML inside `content:encoded` no base, so a reader
/// resolves those against its own origin or not at all, and every image and
/// internal link in the feed arrives broken.
fn absolutise(html: &str) -> String {
    const ATTRIBUTES: [&str; 2] = ["src=\"/", "href=\"/"];

    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    loop {
        // Whichever attribute comes first, so one pass covers both.
        let found = ATTRIBUTES
            .iter()
            .filter_map(|attribute| rest.find(attribute).map(|at| (at, *attribute)))
            .min();
        let Some((at, attribute)) = found else {
            out.push_str(rest);
            return out;
        };

        // Everything up to and including the opening quote.
        let slash = at + attribute.len() - 1;
        out.push_str(&rest[..slash]);
        rest = &rest[slash + 1..];

        // A protocol-relative `//host/path` already carries an origin.
        if !rest.starts_with('/') {
            out.push_str(SITE_URL);
        }
        out.push('/');
    }
}

/// Convert an RFC 3339 timestamp to the RFC 822 form RSS wants.
///
/// Returns `None` for anything chrono cannot parse, which drops just that
/// item's `pubDate` rather than failing the whole feed.
fn rfc822(published: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(published)
        .ok()
        .map(|at| at.to_rfc2822())
}

/// Render the feed.
///
/// Split out from the handler so a test can assert over the XML without
/// standing up a server.
#[must_use]
pub fn render() -> String {
    let mut out = String::from(
        r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
"#,
    );

    out.push_str(&format!("    <title>{}</title>\n", escape(TITLE)));
    out.push_str(&format!("    <link>{SITE_URL}</link>\n"));
    out.push_str(&format!(
        "    <description>{}</description>\n",
        escape(DESCRIPTION)
    ));
    out.push_str("    <language>en-us</language>\n");
    out.push_str(&format!(
        "    <atom:link href=\"{SITE_URL}/rss\" rel=\"self\" type=\"application/rss+xml\" />\n"
    ));

    for post in content::published(Kind::Blog) {
        let url = format!("{SITE_URL}/{}", post.slug);
        out.push_str("    <item>\n");
        out.push_str(&format!("      <title>{}</title>\n", escape(post.title)));
        out.push_str(&format!("      <link>{url}</link>\n"));
        // Slugs are unique and permanent, so the URL doubles as the guid.
        out.push_str(&format!("      <guid isPermaLink=\"true\">{url}</guid>\n"));
        if let Some(at) = post.published.and_then(rfc822) {
            out.push_str(&format!("      <pubDate>{at}</pubDate>\n"));
        }
        if let Some(category) = post.category {
            out.push_str(&format!(
                "      <category>{}</category>\n",
                escape(category)
            ));
        }
        // The whole rendered post, so a reader can show it without a round trip.
        // A `]]>` in the body closes the section early, so it is split across
        // two: entity references are not decoded inside CDATA, and `]]&gt;`
        // would reach the reader as those six literal characters.
        out.push_str(&format!(
            "      <content:encoded><![CDATA[{}]]></content:encoded>\n",
            absolutise(post.body_html).replace("]]>", "]]]]><![CDATA[>")
        ));
        out.push_str("    </item>\n");
    }

    out.push_str("  </channel>\n</rss>\n");
    out
}

/// `GET /rss`.
///
/// Sets its own `Cache-Control`. `cache_control_middleware` keys off the path
/// extension and `/rss` has none, so without this the response carries no cache
/// header at all and a reader polling on a timer rebuilds the whole corpus every
/// hit. The feed only changes on redeploy.
pub async fn handler() -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/rss+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render(),
    )
        .into_response()
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn every_published_blog_post_appears_once() {
        let feed = render();
        for post in content::published(Kind::Blog) {
            let url = format!("{SITE_URL}/{}", post.slug);
            assert_eq!(
                feed.matches(&format!("<link>{url}</link>")).count(),
                1,
                "{} is not in the feed exactly once",
                post.slug
            );
        }
    }

    #[test]
    fn drafts_and_projects_stay_out() {
        let feed = render();
        for post in content::POSTS {
            let listed = post.kind == Kind::Blog && !post.draft && post.published.is_some();
            let url = format!("<link>{SITE_URL}/{}</link>", post.slug);
            assert_eq!(
                feed.contains(&url),
                listed,
                "{} is in the feed when it should not be",
                post.slug
            );
        }
    }

    #[test]
    fn titles_are_escaped() {
        // Every "Lost and Found" title carries literal double quotes, which are
        // not legal as raw text inside an element.
        let feed = render();
        assert!(feed.contains("&quot;Lost and Found&quot;"));
        assert!(!feed.contains("<title>\"Lost"));
    }

    #[test]
    fn root_relative_links_gain_an_origin() {
        assert_eq!(
            absolutise(r#"<img src="/images/a.webp"> and <a href="/tetanes-part-2">two</a>"#),
            format!(
                r#"<img src="{SITE_URL}/images/a.webp"> and <a href="{SITE_URL}/tetanes-part-2">two</a>"#
            )
        );
        // An absolute or protocol-relative link already carries an origin.
        assert_eq!(
            absolutise(r#"<a href="https://example.com/x">x</a><a href="//cdn/x">y</a>"#),
            r#"<a href="https://example.com/x">x</a><a href="//cdn/x">y</a>"#
        );
        // An anchor is relative to the reader's rendering of the item, not to a
        // page, so it is left alone.
        assert_eq!(
            absolutise(r##"<a href="#intro">i</a>"##),
            r##"<a href="#intro">i</a>"##
        );
    }

    #[test]
    fn every_post_body_reaches_the_feed_with_absolute_links() {
        let feed = render();
        // The channel links are built from SITE_URL already, so a body is the
        // only place a root-relative link can survive.
        assert!(!feed.contains(r#"src="/"#), "a relative src is in the feed");
        assert!(
            !feed.contains(r#"href="/"#),
            "a relative href is in the feed"
        );
    }

    #[test]
    fn a_cdata_terminator_in_a_body_is_split_rather_than_entity_escaped() {
        // Entity references are not decoded inside CDATA, so `]]&gt;` would
        // reach the reader as those six characters.
        let split = "]]]]><![CDATA[>";
        assert_eq!("a ]]> b".replace("]]>", split), "a ]]]]><![CDATA[> b");
        assert!(!render().contains("]]&gt;"));
    }

    #[test]
    fn dates_are_rfc_822() {
        assert_eq!(
            rfc822("2020-01-31T21:19:14Z").as_deref(),
            Some("Fri, 31 Jan 2020 21:19:14 +0000")
        );
        // A malformed date drops its own pubDate rather than failing the feed.
        assert_eq!(rfc822("not a date"), None);
    }
}
