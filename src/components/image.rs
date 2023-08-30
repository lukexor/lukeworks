//! Image.

use leptos::{component, html::Img, view, IntoAttribute, IntoView, NodeRef, Scope};

/// Image component.
#[component]
fn Image(cx: Scope, _ref: NodeRef<Img>, width: u32, height: u32, src: String) -> impl IntoView {
    view! { cx,
        <img
            _ref=_ref
            width=width
            height=height
            src=move || format!("/_vercel/image?url={src}&w={width}&q=75")
        />
    }
}
