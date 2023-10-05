//! HTTP static file server.

use crate::portfolio::Portfolio;
use axum::{
    body::{boxed, Body},
    extract::State,
    http::{Request, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use leptos::{view, LeptosOptions};
use leptos_axum::render_app_to_stream;
use tower::ServiceExt;
use tower_http::services::ServeDir;

/// Serve a static file request, falling back to the portfolio homepage if not found.
pub async fn serve(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let file_req = match Request::builder().uri(uri).body(Body::empty()) {
        Ok(req) => req,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to create request: {err}"),
            )
                .into_response()
        }
    };
    let resp = match ServeDir::new(root).oneshot(file_req).await {
        Ok(resp) => resp.map(boxed),
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {err}"),
            )
                .into_response()
        }
    };
    if resp.status() == StatusCode::OK {
        resp.into_response()
    } else {
        let handler = render_app_to_stream(options, move || view! {  <Portfolio /> });
        handler(req).await.into_response()
    }
}
