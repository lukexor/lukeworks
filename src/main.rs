//! `lukeworks.tech` SSR server.

#![doc = include_str!("../README.md")]

#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::server::{cache_control_middleware, cors_middleware};
    use axum::{Router, extract::DefaultBodyLimit, middleware};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use lukeworks::lukeworks::*;
    use tower::limit::ConcurrencyLimitLayer;
    use tower_http::{compression::CompressionLayer, trace::TraceLayer};

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(LukeWorks);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(middleware::from_fn(cache_control_middleware))
        .layer(CompressionLayer::new())
        .layer(cors_middleware())
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)) // 1GB
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(100));

    tracing::info!("lukeworks.tech listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
