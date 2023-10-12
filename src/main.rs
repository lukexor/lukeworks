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

/// The primary SSR entrypoint.
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::{extract::MatchedPath, http::Request, routing::post, Router};
    use leptos_axum::{handle_server_fns, LeptosRoutes};
    use lukeworks::{
        file_server,
        portfolio::{register_server_functions, Portfolio},
    };
    use tower_http::trace::TraceLayer;

    let _guard = lukeworks::tracing_init();

    let conf = leptos::get_configuration(None).await?;
    let options = conf.leptos_options;
    let addr = options.site_addr;

    register_server_functions();

    let routes = leptos_axum::generate_route_list(Portfolio);
    let app = Router::new()
        .route("/api/*fn", post(handle_server_fns))
        .leptos_routes(&options, routes, Portfolio)
        .fallback(file_server::serve)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    uri = ?request.uri(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(options);

    tracing::info!("lukeworks.tech listening on http://{addr}", addr = addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// The primary client entrypoint.
#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side functionality
}
