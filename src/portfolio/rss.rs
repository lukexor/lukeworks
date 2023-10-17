//! RSS feed.

use crate::portfolio::data::{
    BlogPost, ProjectPost, SiteMeta, BLOG_POSTS, LAYOUT, META, PROJECT_POSTS,
};
use axum::response::IntoResponse;
use uuid::Uuid;

struct RssEntry {
    pub guid: Uuid,
    pub link: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub pub_date: &'static str,
}

impl RssEntry {
    fn to_item(&self) -> String {
        let Self {
            guid,
            link,
            title,
            description,
            pub_date,
        } = self;
        format!(
            r#"
        <item>
            <guid>{guid}</guid>
            <link>https://lukeworks.tech/{link}</link>
            <title><![CDATA[{title}]]></title>
            <description><![CDATA[{description}]]></description>
            <pubDate>{pub_date}</pubDate>
        </item>"#,
        )
    }
}

impl From<&'static BlogPost> for RssEntry {
    fn from(post: &'static BlogPost) -> Self {
        Self {
            guid: post.meta.id,
            link: post.meta.slug,
            title: post.meta.title,
            description: post.meta.description,
            pub_date: post.published.as_deref().unwrap_or_default(),
        }
    }
}

impl From<&'static ProjectPost> for RssEntry {
    fn from(post: &'static ProjectPost) -> Self {
        Self {
            guid: post.meta.id,
            link: post.meta.slug,
            title: post.meta.title,
            description: post.meta.description,
            pub_date: &*post.started,
        }
    }
}

/// RSS feed xml.
pub async fn feed() -> impl IntoResponse {
    let entries = BLOG_POSTS
        .iter()
        .map(RssEntry::from)
        .chain(PROJECT_POSTS.iter().map(RssEntry::from))
        .map(|r| r.to_item())
        .collect::<String>();
    let SiteMeta {
        title,
        description,
        origin,
    } = META;
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
        rss_path = LAYOUT.icons.rss.href,
    )
}
