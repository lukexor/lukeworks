//! `lukeworks.tech` SSR server.

#![doc = include_str!("../README.md")]

#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::server::{cache_control_middleware, cors_middleware, redirect_middleware};
    use axum::{Router, extract::DefaultBodyLimit, middleware, routing::get_service};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use lukeworks::lukeworks::*;
    use tower::limit::ConcurrencyLimitLayer;
    use tower_http::{compression::CompressionLayer, services::ServeFile, trace::TraceLayer};

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(LukeWorks);

    // Assets sitting at the root of the site directory need a literal route
    // each. `generate_route_list` registers the bare `/{post}` param route, and
    // a fallback only runs when nothing matched at all, so `/robots.txt` and
    // friends would otherwise be claimed by the post handler and answered with
    // a 404. A literal segment beats a param in the router's matcher, so these
    // win. Anything nested (`/images/…`, `/pkg/…`) has two segments and never
    // collided in the first place.
    let mut app = Router::new();
    for entry in std::fs::read_dir(&*leptos_options.site_root)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(ToOwned::to_owned) else {
            continue;
        };
        tracing::debug!("serving root asset /{name}");
        app = app.route(
            &format!("/{name}"),
            get_service(ServeFile::new(entry.path())),
        );
    }

    let app = app
        // Not a Leptos route: the response is XML, so there is no shell to
        // render. Registered before `leptos_routes` for the same reason the
        // root assets are, since `/{post}` would otherwise claim it.
        .route("/rss", axum::routing::get(lukeworks::feed::handler))
        // Likewise not a Leptos route: a bare document for an <iframe>, with no
        // app around it to hydrate.
        .route(
            "/sketch/{name}",
            axum::routing::get(lukeworks::sketch::handler),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(middleware::from_fn(cache_control_middleware))
        // Layers wrap outward, so this runs before routing and short-circuits
        // old URLs without touching `leptos_routes` or the static-file handler.
        .layer(middleware::from_fn(redirect_middleware))
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
