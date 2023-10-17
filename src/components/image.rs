//! Image.

use leptos::*;
use leptos_dom::html::Img;

/// Image component.
#[component]
fn Image(_ref: NodeRef<Img>, width: u32, height: u32, src: String) -> impl IntoView {
    view! {
        <img
            _ref=_ref
            width=width
            height=height
            src=move || format!("/_vercel/image?url={src}&w={width}&q=75")
        />
    }
}
