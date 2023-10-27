//! Portfolio models.

use crate::types::{DateStr, ImgAttrs, Url};
use serde::Deserialize;
use std::collections::HashSet;

/// Portfolio post metadata.
#[derive(Deserialize, Debug, Clone)]
#[must_use]
pub struct PostMeta {
    pub slug: Url,
    pub title: String,
    pub description: String,
    pub thumbnail: ImgAttrs,
    pub category: String,
    pub tags: HashSet<String>,
}

/// Blog Post.
#[derive(Deserialize, Debug, Clone)]
#[must_use]
pub struct BlogPost {
    pub meta: PostMeta,
    pub minutes_to_read: u32,
    pub published: Option<DateStr>,
}

/// Project Post.
#[derive(Deserialize, Debug, Clone)]
#[must_use]
pub struct ProjectPost {
    pub meta: PostMeta,
    pub website: Option<Url>,
    pub started: DateStr,
    pub completed: Option<DateStr>,
}
