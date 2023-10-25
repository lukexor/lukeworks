//! Image.

use leptos::*;
use std::borrow::Cow;

/// Image component.
#[component]
pub fn Img(
    width: u32,
    height: u32,
    #[prop(into)] src: Cow<'static, str>,
    node_ref: Option<NodeRef<html::Img>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    // TODO: See if this can be de-duped
    if let Some(node_ref) = node_ref {
        view! {
            <img
                width=width
                height=height
                node_ref=node_ref
                src=move || format!("/_vercel/image?url={src}&w={width}&q=75")
                {..attrs}
            />
        }
    } else {
        view! {
            <img
                width=width
                height=height
                src=move || format!("/_vercel/image?url={src}&w={width}&q=75")
                {..attrs}
            />
        }
    }
}
