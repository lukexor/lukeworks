use crate::lukeworks::errors::NotFound;
use chrono::{DateTime, Utc};
use leptos::{component, view, IntoView, Scope, SignalWith};
use leptos_router::use_params_map;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Url(String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Image {
    src: Url,
    alt: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Entry {
    name: String,
    title: String,
    thumbnail: Image,
    image: Image,
    category: String,
    tags: HashSet<String>,
    minutes_to_read: u32,
    likes: u32,
    published: Option<DateTime<Utc>>,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum PostType {
    Blog(Entry),
    Project {
        entry: Entry,
        website: Option<Url>,
        started: DateTime<Utc>,
        completed: Option<DateTime<Utc>>,
    },
}

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let post = params.with(|param| param.get("post").cloned().unwrap_or_default());
    if post.is_empty() {
        view! { cx, <NotFound /> }
    } else {
        view! { cx,
            "Post"
        }
        .into_view(cx)
    }
}
