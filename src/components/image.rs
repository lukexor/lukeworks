use leptos::prelude::*;

/// Image component.
#[component]
pub fn Image(src: TextProp, title: TextProp, alt: TextProp) -> impl IntoView {
    view! { <image src=src title=title alt=alt /> }
}
