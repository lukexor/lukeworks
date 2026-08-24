//! The p5.js sketches nine project posts embed.
//!
//! Served by a plain Axum handler rather than a Leptos route. The response is a
//! bare document that loads p5 and one sketch, it only ever renders inside an
//! `<iframe>`, and it shares nothing with the app. Hydrating the WASM bundle
//! into it would download 120KB to draw a canvas.
//!
//! The sketches themselves live under `public/sketch/js/` as one readable ES
//! module each, edited in place. They are part of what the project posts show
//! off, so they stay as source a reader can follow rather than a build output.
//! `js/utils.js` is the click-to-start helper they share. p5 loads as a global
//! `<script>` ahead of them, so `p5.Vector` resolves without an import.
//!
//! `lib/` contains p5 1.11.13 and its licence. The sketches are written against
//! p5 1.x and 2.x renamed enough of the API to break them, so the pin is a
//! major version, not a preference.

use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};

/// Every sketch that has a module in `public/sketch/js/`.
///
/// An allowlist rather than a directory read: `name` arrives from the URL and
/// is interpolated into a `<script src>`, so anything outside this list has to
/// 404 rather than reach the filesystem.
///
/// This route owns every single-segment path under `/sketch/`, which is why p5
/// and the font sit in `public/sketch/lib/`. As siblings they resolved here
/// first, failed the allowlist, and 404ed instead of being served.
pub const SKETCHES: [&str; 9] = [
    "asteroids",
    "fireworks",
    "fluid-simulation",
    "fourier",
    "lorenz-attractor",
    "matrix",
    "maze-astar",
    "pong",
    "raycasting-2d",
];

/// Render the page for one sketch.
///
/// `name` is already known to be in [`SKETCHES`], so it needs no escaping.
fn page(name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{name}</title>
<meta name="robots" content="noindex">
<style>
  /* A sketch may build its own controls with p5's createButton and createP,
     which land in <body> under the canvas and inherit from here. Without a
     colour they are black text on a black page: maze-astar prints its timings
     into one, and they were invisible rather than missing. Every embed sets
     `scrolling="no"`, so anything past the frame height is unreachable: a
     sketch with controls under the canvas needs a taller iframe in the post. */
  html, body {{ margin: 0; padding: 0; background: #000; color: #d7e3ec; }}
  body {{ font: 14px/1.5 ui-sans-serif, system-ui, sans-serif; }}
  canvas {{ display: block; }}
  button {{ font: inherit; padding: 4px 10px; }}
</style>
</head>
<body>
<script src="/sketch/lib/p5.min.js"></script>
<script type="module">
  // Each sketch default-exports its p5 instance-mode function. Modules are
  // deferred, so the classic script above has already defined the p5 global by
  // the time this runs. p5 draws into <body> when handed no container.
  import sketch from "/sketch/js/{name}.js";
  new p5(sketch);
</script>
<script>
  // A sketch owns the arrow keys and the space bar. Left to the browser they
  // scroll the page this iframe sits in, out from under the canvas mid-game.
  //
  // Capture phase, because p5 binds its own keydown on window and a sketch that
  // stops propagation would otherwise get here first. Preventing the default
  // does not stop p5 reading the key: it only cancels the scroll.
  //
  // A sketch's own controls are exempt: space is how a keyboard user activates
  // a focused <button>, and maze-astar builds two of them.
  var held = ["Space", "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];
  addEventListener("keydown", function (event) {{
    var tag = event.target && event.target.tagName;
    if (tag === "BUTTON" || tag === "INPUT" || tag === "SELECT" || tag === "TEXTAREA") {{
      return;
    }}
    if (held.indexOf(event.code) > -1) {{
      event.preventDefault();
    }}
  }}, true);

  // Keys only reach this document once it has focus. A mousedown normally hands
  // focus to the frame, but p5 cancels that default, so the frame takes focus
  // itself and a click on the canvas is enough to start playing.
  addEventListener("pointerdown", function () {{
    window.focus();
  }}, true);
</script>
</body>
</html>
"#
    )
}

/// `GET /sketch/{{name}}`.
pub async fn handler(Path(name): Path<String>) -> Response {
    if !SKETCHES.contains(&name.as_str()) {
        return StatusCode::NOT_FOUND.into_response();
    }

    (
        StatusCode::OK,
        // Same reasoning as the feed: `cache_control_middleware` keys off the
        // path extension and this route has none, so it sets its own. The page
        // is a constant. The `.js` files it names get their headers from that
        // middleware, since those paths do have an extension.
        [(header::CACHE_CONTROL, "public, max-age=3600")],
        Html(page(&name)),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{self, Kind};

    #[test]
    fn every_embedded_sketch_has_a_route() {
        // Nine project posts embed `<iframe src="/sketch/...">`. A sketch named
        // in a body but missing from the allowlist renders as a 404 inside the
        // post, which is easy to miss by eye.
        for post in content::published(Kind::Project) {
            for (at, _) in post.body_html.match_indices("/sketch/") {
                let rest = &post.body_html[at + "/sketch/".len()..];
                let name = rest
                    .split(['"', '\'', '<', ' ', '?', '#'])
                    .next()
                    .unwrap_or_default();
                assert!(
                    SKETCHES.contains(&name),
                    "{} embeds /sketch/{name}, which is not in SKETCHES",
                    post.slug
                );
            }
        }
    }

    #[test]
    fn the_page_names_its_own_sketch() {
        let page = page("matrix");
        assert!(page.contains(r#"import sketch from "/sketch/js/matrix.js";"#));
        assert!(page.contains(r#"<script src="/sketch/lib/p5.min.js"></script>"#));
    }
}
