#![doc = include_str!("../README.md")]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)] // Too many false positives

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::Router;
    use leptos::view;
    use leptos_axum::LeptosRoutes;
    use lukeworks::{file_server, lukeworks::LukeWorks};
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(
                    "lukeworks=info"
                        .parse()
                        .expect("failed to parse env filter"),
                )
                .from_env_lossy(),
        )
        .init();

    let conf = leptos::get_configuration(None).await?;
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = leptos_axum::generate_route_list(|cx| view! { cx, <LukeWorks /> }).await;

    let app = Router::new()
        .leptos_routes(&options, routes, |cx| view! { cx, <LukeWorks /> })
        .fallback(file_server::serve)
        .with_state(options);

    tracing::info!("lukeworks listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}
