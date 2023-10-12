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
    use portfolio::Portfolio;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(Portfolio);
}

/// Initialize trace logging.
#[cfg(feature = "ssr")]
pub fn tracing_init() -> tracing_appender::non_blocking::WorkerGuard {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "lukeworks=debug,tower_http=debug,axum::rejection=trace".into());

    let registry = tracing_subscriber::registry().with(env_filter).with(
        fmt::layer()
            .compact()
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_writer(std::io::stderr),
    );

    let file_appender = tracing_appender::rolling::daily("logs", "lukeworks.log");
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
    let registry = registry.with(
        fmt::Layer::new()
            .compact()
            .json()
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_writer(non_blocking),
    );

    if let Err(err) = registry.try_init() {
        eprintln!("setting tracing default failed: {err:?}");
    }

    worker_guard
}
