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

## Content pipeline (unfinished)

`content/*.toml` and `content/posts/*.md` hold the site data, but **nothing parses them yet**:
there is no `serde`/`toml` dependency, the TOML files still carry JSON-isms (trailing commas,
`null`, multi-line inline tables) and would not parse as-is, and `find_post` in
`src/pages/post.rs` is a hardcoded stub around an unused `include_str!`. `comrak` is a dependency
for eventual markdown rendering but is not wired in. Expect to build this out rather than extend it.

## Migration

`MIGRATION.md` holds the plan for finishing the Next.js → Leptos port (target: SSR + islands,
build-time markdown, Rust-rewritten sketches). Read it before starting feature work — several
decisions there supersede what's currently in the tree, notably that `content/*.toml` is being
replaced by frontmatter and that islands mode changes how `use_theme` must work.

The pre-port Next.js source is not in the working tree; it's in git at `d82ff04^`
(`git ls-tree -r --name-only d82ff04^ -- web/src`) and is the reference for old behavior.

## Current state

The branch is mid-restructure (`wip` commits). Most of `src/pages/` and `src/components/` are
stubs, only `/`, `/about`, and `/:post` are routed, and some class names in `src/lukeworks.rs`
(e.g. `text-brand-fg2`) refer to design tokens that no longer exist in `style/tailwind.css`.

`web/` is untracked leftover node_modules/build output from the previous Next.js site. It is not
part of the build — ignore it.

## Deployment

`.github/workflows/ci.yml` runs lint → audit → test → build on push/PR to `main`; the deploy step
is still a TODO. Release builds should set `LEPTOS_HASH_FILES=true` — `RootStylesheet` in
`src/lukeworks.rs` switches between `HashedStylesheet` and a plain `Stylesheet` on that option.
