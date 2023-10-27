//! RSS feed.

use crate::{
    portfolio::{
        constants::{meta, BLOG_POSTS, PROJECT_POSTS},
        icons,
        models::{BlogPost, ProjectPost},
    },
    types::{DateStr, Url},
};
use axum::response::IntoResponse;

struct RssEntry {
    pub slug: Url,
    pub title: String,
    pub description: String,
    pub pub_date: DateStr,
}

impl RssEntry {
    fn to_item(&self) -> String {
        let Self {
            slug,
            title,
            description,
            pub_date,
        } = self;
        format!(
            r#"
        <item>
            <guid>https://lukeworks.tech/{slug}</guid>
            <link>https://lukeworks.tech/{slug}</link>
            <title><![CDATA[{title}]]></title>
            <description><![CDATA[{description}]]></description>
            <pubDate>{pub_date}</pubDate>
        </item>"#,
        )
    }
}

#[derive(Debug)]
struct MissingPubDate(Url);

impl TryFrom<BlogPost> for RssEntry {
    type Error = MissingPubDate;

    fn try_from(post: BlogPost) -> Result<Self, Self::Error> {
        match post.published {
            Some(pub_date) => Ok(Self {
                slug: post.meta.slug,
                title: post.meta.title,
                description: post.meta.description,
                pub_date,
            }),
            None => Err(MissingPubDate(post.meta.slug)),
        }
    }
}

impl From<ProjectPost> for RssEntry {
    fn from(post: ProjectPost) -> Self {
        Self {
            slug: post.meta.slug,
            title: post.meta.title,
            description: post.meta.description,
            pub_date: post.started,
        }
    }
}

/// RSS feed xml.
pub async fn feed() -> impl IntoResponse {
    let entries = BLOG_POSTS
        .iter()
        .cloned()
        .filter_map(|post| RssEntry::try_from(post).ok())
        .chain(PROJECT_POSTS.iter().cloned().map(RssEntry::from))
        .map(|r| r.to_item())
        .collect::<String>();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{title}</title>
        <description>{description}</description>
        <link>{origin}</link>
        <language>en-us</language>
        <ttl>60</ttl>
        <atom:link href="{origin}/{rss_path}" rel="self" type="application/rss+xml" />
        {entries}
    </channel>
</rss>"#,
        title = meta::TITLE,
        description = meta::DESC,
        origin = meta::ORIGIN,
        rss_path = icons::RSS,
    )
}
