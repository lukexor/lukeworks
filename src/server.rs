use axum::{
    extract::Request,
    http::{HeaderValue, Method, header},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use std::path::Path;
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Serve the permanent redirects inherited from the Next.js site.
///
/// Runs ahead of routing so an old URL never reaches `FlatRoutes` — several of
/// them would otherwise be caught by the bare `/:post` param segment and render
/// a 404 instead of forwarding.
///
/// The query string is carried across: these are indexed URLs and campaign or
/// referral parameters hang off them.
pub async fn redirect_middleware(request: Request, next: Next) -> Response {
    // `server` is a module of the binary crate; the redirect table lives in the
    // library, hence the crate name rather than `crate::`.
    if let Some(path) = lukeworks::redirects::resolve(request.uri().path()) {
        let location = match request.uri().query() {
            Some(query) if !query.is_empty() => format!("{path}?{query}"),
            _ => path,
        };
        // 308 rather than 301: both are permanent, but 308 forbids the method
        // rewriting that some clients still do on 301. Every rule here targets
        // a GET page, so the distinction is about correctness for crawlers and
        // caches rather than anything a browser will show.
        return Redirect::permanent(&location).into_response();
    }

    next.run(request).await
}

pub async fn cache_control_middleware(request: Request, next: Next) -> Response {
    let should_cache = Path::new(request.uri().path())
        .extension()
        .and_then(|ext| ext.to_str())
        // `js` belongs here with `wasm`: wasm-bindgen emits the glue and the
        // module as a matched pair keyed on mangled export names. Leaving `js`
        // off the list gave it no cache-control at all, so the browser applied
        // heuristic caching and paired a stale glue file with a fresh module —
        // "wasm.wasm_bindgen__convert__closures_____invoke__… is not a function".
        .map(|ext| ["css", "ico", "js", "wasm", "webp", "woff2"].contains(&ext))
        .unwrap_or(false);

    let mut response = next.run(request).await;
    if should_cache {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            // The sense of this test was inverted: dev builds were handing out
            // `immutable, max-age=30d` for `.wasm`, so `cargo leptos watch`
            // rebuilt while the browser kept replaying the cached bundle — edits
            // appeared not to take until a manual cache-bypassing reload.
            if cfg!(debug_assertions) {
                // Don't cache in development.
                HeaderValue::from_static("no-store")
            } else {
                // Release serves hashed filenames - 30 days.
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
