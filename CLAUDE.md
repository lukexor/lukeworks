# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`lukeworks.tech` — a personal blog/portfolio built with [Leptos](https://github.com/leptos-rs/leptos) 0.8 (SSR + WASM hydration) and Tailwind CSS 4, served by an Axum binary.

## Commands

All day-to-day work goes through `just` (see `justfile`):

```
just install   # cargo-binstall just, cargo-deny, cargo-leptos, cargo-nextest
just dev       # cargo leptos watch --split  (dev server w/ hot reload)
just run       # cargo leptos serve --release --split
just build     # cargo leptos build --release --split
just test      # cargo leptos test && cargo nextest run
just audit     # cargo deny check (licenses + advisories)
```

Lint locally the way CI does:

```
cargo fmt --all --check
cargo clippy --all-targets --all-features --keep-going
```

Single test: `cargo nextest run <substring>` or `cargo nextest run -E 'test(name_of_test)'`.
There are currently no tests in the tree.

Toolchain is pinned by `rust-toolchain.toml` (1.91.1, edition 2024, `wasm32-unknown-unknown`).
`.cargo/config.toml` links with `mold` on Linux — it must be installed.

## Rendering model: SSR + full hydration

Standard Leptos SSR with client hydration. Component bodies run on the server **and again in the
browser during hydration**, which drives one non-obvious rule:

**Never read `crate::content::POSTS` from a component.** The compiled post table is
`#[cfg(feature = "ssr")]`-gated to keep ~124KB of rendered HTML out of the WASM bundle, so under
`hydrate` it is an empty slice. A component reading it directly renders correctly on the server
and then blanks out during hydration. Go through a server function instead —
`pages::post::fetch_post` is the pattern: leptos serializes the resolved `Resource` value into the
page, so hydration costs no extra request and only client-side navigation to a *new* post fetches.

The tradeoff is that a post's HTML appears twice in the response (rendered DOM + serialized
resource). Raw size roughly +40%, but the copies are near-identical text so gzip absorbs most of
it — a large post is ~21KB on the wire.

**Islands mode was tried and abandoned** (see `MIGRATION.md`). It cut the WASM bundle from 120KB
to 42KB gzipped, but `islands-router` left stale content after navigation and the theme toggle
never worked. If you reconsider it, the constraints were: `#[component]` bodies never run in the
browser, island props must be `Serialize`, and `<For>` keys must be `Serialize`.

`wasm-bindgen` is a non-optional dependency because component bodies that touch `web_sys` must
still typecheck under `ssr`.

## The two-target build

This is a single crate that compiles **twice**, and that shapes almost everything:

- `--features ssr` → `src/main.rs` builds the Axum server binary (`bin-features` in
  `[package.metadata.leptos]`).
- `--features hydrate` → `src/lib.rs` builds the WASM bundle; `hydrate()` is the
  `wasm_bindgen` entrypoint that mounts `LukeWorks` onto the already-rendered body.

Consequences when adding code:

- Every module in `src/lib.rs` must compile under *both* feature sets. Server-only code
  (anything touching `axum`, `tokio`, `tower`) needs `#[cfg(feature = "ssr")]`; browser-only
  code (`web_sys`, `document()`, `window()`) needs `#[cfg(not(feature = "ssr"))]`.
  `src/hooks/use_theme.rs` is the canonical example of branching on both sides.
- A bare `cargo check` compiles with neither feature and proves very little. Use
  `cargo clippy --all-features` (what CI runs) or check one target explicitly.
- Release WASM uses the separate `wasm-release` profile (`opt-level = 'z'`, `strip = false`).

## Layout

- `src/lukeworks.rs` — the app root. Holds `shell()` (the full HTML document rendered by the
  server), the `LukeWorks` component (meta context, theme, `Router`/`FlatRoutes`), plus the
  `ROUTES` const and `SUPPORT_EMAIL`. New routes are registered in the `FlatRoutes` block here.
- `src/main.rs` — Axum wiring: leptos routes, compression, CORS, cache-control, concurrency limit.
- `src/server.rs` — the two custom middlewares (`cache_control_middleware`, `cors_middleware`).
- `src/pages/`, `src/components/`, `src/hooks/` — declared inline as `pub mod` blocks in
  `src/lib.rs` (no `mod.rs` files). Add new files there.
- `content/` — post markdown plus TOML metadata (see below).
- `style/tailwind.css` — the real stylesheet; `style/main.css` is the `style-file` leptos expects
  and is currently empty.

## Styling and theme

Tailwind 4, so the theme lives in CSS, not JS: design tokens are `@theme` custom properties in
`style/tailwind.css` (`--color-primary`, `--color-action-1`, …). `tailwind.config.js` exists
*only* to keep `tailwindcss-intellisense` working — do not add config there expecting it to apply.

Dark mode is a custom variant: `@custom-variant dark (&:where(.dark, .dark *))`, driven by a
`.dark` class on `<body>`. `use_theme()` in `src/hooks/use_theme.rs` owns this:

- Default is **dark**. Initial value comes from the `sec-ch-prefers-color-scheme` header on the
  server and `matchMedia` in the browser; a `prefers-dark` cookie overrides both once toggled.
- Because `<body>` and the `color-scheme` meta tag live outside the reactive tree, an `Effect`
  imperatively syncs them. Anything else needing the theme should call `use_prefers_dark()`,
  which reads the provided `Theme` context.

Stylelint config (`.stylelintrc.json`) targets the CSS; there is no npm project at the repo root
to run it from.

## Content pipeline

`build.rs` compiles `content/posts/*.md` into a static `POSTS` table: it parses YAML frontmatter
(`gray_matter`), renders markdown to HTML (`comrak` + syntect highlighting), derives reading time,
sorts newest-first, and writes a generated module to `OUT_DIR`. `src/content.rs` includes it and
adds `find`/`published` helpers. See `content/README.md` for the frontmatter schema.

Consequences worth knowing:

- comrak and gray_matter are **`[build-dependencies]`** — no markdown machinery ships in either
  binary, and the server does no I/O and no parsing at runtime.
- The generated table is `#[cfg(feature = "ssr")]`; under `hydrate`, `POSTS` is an empty slice so
  post bodies stay out of the WASM bundle. Adding a `#[component]` that reads `POSTS` is fine;
  an **island** that needs post data must go through a server function instead.
- Adding a post means adding one markdown file. There is no index to update.
- `content/portfolio.toml` (site copy) and `content/redirects.toml` are hand-maintained data,
  not generated.

## Migration

`MIGRATION.md` holds the plan for finishing the Next.js → Leptos port (target: SSR + islands,
build-time markdown, Rust-rewritten sketches). Read it before starting feature work — several
decisions there supersede what's currently in the tree, notably that `content/*.toml` is being
replaced by frontmatter and that islands mode changes how `use_theme` must work.

The pre-port Next.js source is not in the working tree; it's in git at `d82ff04^`
(`git ls-tree -r --name-only d82ff04^ -- web/src`) and is the reference for old behavior.

## Current state

Phases 0–3 of `MIGRATION.md` are done. Content, theming, and islands work end to end; both
targets build clean with no warnings.

Still stubs: `Home`, `About`, `Blog`, `Projects`, `Resume`, `Search`, `TetanesWeb`, and the
`Button`/`Image` components render placeholders. Routed so far: `/`, `/about`, `/blog`,
`/projects`, `/:post`. The `/:post` route is a bare param segment and **must stay last** in
`FlatRoutes` — it matches any single path segment.

Not yet built (Phase 4+): the redirect layer (`content/redirects.toml` is transcribed but unused),
`/rss`, `/sketch/:name` — several posts embed `<iframe src="/sketch/...">` that 404 until then —
`/resume`, `/tetanes-web`, and search.

Styling is deliberately minimal pending the Phase 5 design pass.

## Deployment

`.github/workflows/ci.yml` runs lint → audit → test → build on push/PR to `main`; the deploy step
is still a TODO. Release builds should set `LEPTOS_HASH_FILES=true` — `RootStylesheet` in
`src/lukeworks.rs` switches between `HashedStylesheet` and a plain `Stylesheet` on that option.
