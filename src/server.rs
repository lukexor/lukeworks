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

/// Sets `cache-control` on the static assets, by how bustable their URL is.
///
/// `hash_files` is `LeptosOptions::hash_files`, and it decides whether a
/// `/pkg/` name is safe to pin. Without it the bundle, the split glue and every
/// chunk keep one name across builds, so `immutable` pins a browser to the
/// build it first saw for a month. A main bundle paired with the wrong split
/// glue then fails to instantiate rather than merely looking stale.
pub async fn cache_control_middleware(hash_files: bool, request: Request, next: Next) -> Response {
    let should_cache = Path::new(request.uri().path())
        .extension()
        .and_then(|ext| ext.to_str())
        // `js` has to stay alongside `wasm`: wasm-bindgen emits the glue and the
        // module as a matched pair keyed on mangled export names. An extension
        // missing from this list gets no cache-control at all and falls to the
        // browser's heuristic caching, which will pair stale glue with a fresh
        // module and fail as
        // "wasm.wasm_bindgen__convert__closures_____invoke__... is not a function".
        .map(|ext| {
            [
                "css", "ico", "js", "pdf", "png", "ttf", "wasm", "webp", "woff2",
            ]
            .contains(&ext)
        })
        .unwrap_or(false);

    // Only `/pkg/` filenames carry a content hash, and only when cargo-leptos
    // was given `LEPTOS_HASH_FILES`. Everything else (the images, the fonts,
    // the sketch bundles) keeps its name across deploys, so a new build of one
    // is a new body behind an old URL.
    let hashed = hash_files && request.uri().path().starts_with("/pkg/");

    // The one hashed name that lies. cargo-leptos names the split glue inside
    // the main bundle after hashing that bundle, so two builds differing only
    // in their chunks share a bundle URL over a body naming a glue file that is
    // no longer on disk. Pinned, a returning browser links its cached bundle
    // against chunks that are gone and the module fails. A 304 on 23KB rules
    // that out. `__wasm_split.<hash>.js` is hashed from its final contents, so
    // it stays immutable.
    let stale_hash = hashed
        && Path::new(request.uri().path())
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".js") && !name.starts_with("__wasm_split"));

    let mut response = next.run(request).await;
    // Only a successful body is worth caching. Applying the release header to a
    // 404 pins the failure in the browser for 30 days, so a missing asset stays
    // missing for that visitor long after it is put back.
    if should_cache && response.status().is_success() {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            // Dev filenames are unhashed and `cargo leptos watch` rewrites them
            // in place, so anything cachable here is served stale to a browser
            // that already has it, and edits appear not to take.
            if cfg!(debug_assertions) {
                HeaderValue::from_static("no-store")
            } else if stale_hash {
                HeaderValue::from_static("public, no-cache")
            } else if hashed {
                // A hashed name is a new URL for every new body, so nothing
                // behind this one ever changes.
                HeaderValue::from_static("public, max-age=2592000, immutable")
            } else {
                // A day, and revalidatable. `immutable` here would pin a
                // regenerated sketch bundle or a replaced image in the browser
                // for a month, past a reload, with no URL to change.
                HeaderValue::from_static("public, max-age=86400")
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
