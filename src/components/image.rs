//! Image.

use leptos::*;
use std::borrow::Cow;

/// Image component.
#[component]
pub fn Img(
    width: u32,
    height: u32,
    #[prop(into)] src: Cow<'static, str>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <img
            width=width
            height=height
            src=move || format!("/_vercel/image?url={src}&w={width}&q=75")
            {..attrs}
        />
    }
}
