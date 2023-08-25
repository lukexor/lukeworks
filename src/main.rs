#![doc = include_str!("../README.md")]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)] // Too many false positives

mod file_server;
mod lukeworks;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use crate::lukeworks::LukeWorks;
    use axum::Router;
    use leptos::view;
    use leptos_axum::LeptosRoutes;
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
        .await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}
