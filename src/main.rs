//! `lukeworks.tech` SSR server.

#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
)]

/// The primary SSR entrypoint.
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::Router;
    use leptos::view;
    use leptos_axum::LeptosRoutes;
    use lukeworks::{file_server, portfolio::Portfolio};

    lukeworks::initialize_tracing();

    let conf = leptos::get_configuration(None).await?;
    let options = conf.leptos_options;
    let addr = options.site_addr;

    let routes = leptos_axum::generate_route_list(|| view! {  <Portfolio /> });
    let app = Router::new()
        .leptos_routes(&options, routes, || view! {  <Portfolio /> })
        .fallback(file_server::serve)
        .with_state(options);

    tracing::info!("lukeworks.tech listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Handles graceful shutdown in the event of termination signals.
#[cfg(feature = "ssr")]
async fn shutdown_signal() {
    use tokio::signal::{ctrl_c, unix};

    let ctrl_c = async {
        ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    let terminate = async {
        unix::signal(unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}

/// The primary client entrypoint.
#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side functionality
}
