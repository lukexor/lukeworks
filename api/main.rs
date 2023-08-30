use axum::{
    body::{Bytes, HttpBody},
    Router,
};
use leptos::view;
use leptos_axum::LeptosRoutes;
use lukeworks::{file_server, lukeworks::LukeWorks};
use tower::Service;
use vercel_runtime as vr;

#[tokio::main]
async fn main() -> Result<(), vr::Error> {
    vr::run(handler).await
}

pub async fn handler(req: vr::Request) -> Result<vr::Response<vr::Body>, vr::Error> {
    let conf = leptos::get_configuration(None).await?;
    let options = conf.leptos_options;
    let routes = leptos_axum::generate_route_list(|cx| view! { cx, <LukeWorks /> }).await;
    let mut app = Router::new()
        .leptos_routes(&options, routes, |cx| view! { cx, <LukeWorks /> })
        .fallback(file_server::serve)
        .with_state(options);
    let resp = app.call(req.map(req_into_axum)).await?;
    axum_into_resp(resp).await
}

fn req_into_axum(body: vr::Body) -> axum::body::Body {
    match body {
        vr::Body::Empty => axum::body::Body::empty(),
        vr::Body::Text(str) => str.into(),
        vr::Body::Binary(bytes) => bytes.into(),
    }
}

async fn axum_into_resp(
    resp: axum::response::Response<axum::body::BoxBody>,
) -> Result<vr::Response<vr::Body>, vr::Error> {
    let (parts, mut body) = resp.into_parts();
    let body = body.data().await.unwrap_or_else(|| Ok(Bytes::new()))?;
    Ok(vr::Response::from_parts(
        parts,
        vr::Body::Binary(body.to_vec()),
    ))
}
