/// Portfolio posts.
use crate::portfolio::{
    data::{DateStr, ImageAttrs, Url},
    errors::NotFound,
};
use leptos::{component, view, IntoView, SignalWith};
use leptos_router::use_params_map;
use std::collections::HashSet;
use uuid::Uuid;

/// Portfolio post metadata.
#[derive(Debug, Clone)]
#[must_use]
pub struct Meta {
    pub id: Uuid,
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub thumbnail: ImageAttrs,
    pub category: &'static str,
    pub tags: HashSet<&'static str>,
}

/// Blog Post.
#[derive(Debug, Clone)]
#[must_use]
pub struct Blog {
    pub meta: Meta,
    pub minutes_to_read: u32,
    pub published: Option<DateStr>,
}

/// Project Post.
#[derive(Debug, Clone)]
#[must_use]
pub struct Project {
    pub meta: Meta,
    pub website: Option<Url>,
    pub started: DateStr,
    pub completed: Option<DateStr>,
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let post = params.with(|param| param.get("post").cloned().unwrap_or_default());
    if post.is_empty() {
        view! {  <NotFound /> }
    } else {
        view! {
            "Post"
        }
        .into_view()
    }
}
