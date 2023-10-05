//! `lukeworks.tech` portfolio.

#![warn(
    clippy::all,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
)]

pub mod components;
#[cfg(feature = "ssr")]
pub mod file_server;
pub mod portfolio;

/// Quickly create and populate a [`HashMap`].
#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),+$(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }}
}

#[cfg(feature = "hydrate")]
#[cfg_attr(feature = "hydrate", wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
    use leptos::view;
    use portfolio::Portfolio;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| {
        view! { <Portfolio /> }
    });
}

/// Initialize trace logging.
#[cfg(feature = "ssr")]
pub fn initialize_tracing() {
    use tracing::Level;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy();

    let registry = tracing_subscriber::registry().with(env_filter).with(
        fmt::Layer::new()
            .compact()
            .without_time()
            .with_line_number(true)
            .with_writer(std::io::stderr),
    );

    if let Err(err) = registry.try_init() {
        eprintln!("setting tracing default failed: {err:?}");
    }
}
