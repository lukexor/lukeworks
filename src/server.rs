use axum::{
    extract::Request,
    http::{HeaderValue, Method, header},
    middleware::Next,
    response::Response,
};
use std::path::Path;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub async fn cache_control_middleware(request: Request, next: Next) -> Response {
    let should_cache = Path::new(request.uri().path())
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ["css", "ico", "wasm", "webp", "woff2"].contains(&ext))
        .unwrap_or(false);

    let mut response = next.run(request).await;
    if should_cache {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            // Don't cache in development
            if !cfg!(debug_assertions) {
                HeaderValue::from_static("no-store")
            } else {
                // Release should be using hashes - 30 days
                HeaderValue::from_static("public, max-age=2592000, immutable")
            },
        );
    }
    response
}

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_origin(AllowOrigin::predicate(|origin, _| {
            let bytes = origin.as_bytes();
            bytes.starts_with(b"http://localhost") || bytes.starts_with(b"http://127.0.0.1")
        }))
        // 30 days
        .max_age(std::time::Duration::from_secs(2592000))
}
