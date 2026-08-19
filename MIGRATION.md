# Next.js → Leptos migration plan

Target architecture: **Leptos 0.8 SSR with full hydration**, content compiled to HTML at build
time, single self-contained binary. Islands were tried and abandoned (see below).

The authoritative spec for "what the old site did" is git, not the working tree. The Next.js
source was deleted in `d82ff04` (*refactor: initial leptos version*); read it with:

```
git show d82ff04^:web/src/pages/index.tsx
git ls-tree -r --name-only d82ff04^ -- web/src
```

---

## Status

| Phase | State |
|---|---|
| 0 — Recover lost assets | done (`c494c40`) |
| 1 — Deps + islands | done (`ddb95f1`) |
| 2 — Content pipeline | done (`8fad448`) |
| 3 — Islands conversion | done — theme reworked, toggle island, nav |
| 4 — Routes and features | partial — redirects + listings done; RSS, search, `/resume`, `/sketch/:name`, `/tetanes-web` outstanding |
| 5 — Design | not started |
| 6 — Sketches | not started |
| 7 — Deploy | not started |

### Islands: tried, measured, abandoned

Islands were adopted in Phase 1 to cut WASM size, then dropped after two bugs.

**`islands-router` left stale content after navigation.** Its diff keys off `bo-TypeId(..)`
markers derived from view *types*, so all posts emit identical markers and the branch is never
replaced; it falls back to a node-by-node patch that assumes both pages share a DOM shape.
Markdown bodies rendered via `inner_html` are opaque to the typed view tree, so the walkers desync
and everything past that point stays stale. Reproduced headlessly by running leptos's own
`islands_routing.js` under jsdom against real responses — two posts desynced at node #205 with
zero differing branch markers.

**The theme toggle never worked** under islands, and the cause was never established (no browser
automation was available to diagnose it).

Measured cost of dropping islands, release build (`opt-level='z'` + LTO):

| | raw | gzipped |
|---|---|---|
| islands | 97,755 | 42,292 |
| full hydration | 294,444 | 119,990 |

So ~78KB gzipped is the price of the working, well-trodden path. A third option was considered and
not taken: **SSR with no WASM at all** (0KB on content pages, theme toggle as inline JS, per-page
bundles for the sketches and tetanes-web). That remains the best answer if bundle size ever
becomes the binding constraint — it would mean leaving cargo-leptos's front-end pipeline for a
plain `cargo build` plus a direct `tailwindcss` call.

`cargo-leptos` had to move 0.2.47 → 0.3.7 during Phase 3, not as tidying: 0.2.47
bundles a wasm-bindgen CLI pinned to 0.2.105, and the Phase 1 bump to 0.2.126
(forced by leptos-use 0.19) made every build fail with a bindgen schema
mismatch. 0.3.x resolves the CLI from the lockfile instead. Anyone building this
tree needs `just install` first.

## Decisions

| Area | Decision |
|---|---|
| Post metadata | YAML frontmatter in each `content/posts/*.md`; delete `content/*.toml` |
| Rendering | Markdown → HTML at **build time** (`build.rs` + comrak), embedded in the server binary |
| Interactivity | SSR + full hydration. Islands tried and abandoned — see below |
| Navigation | Client-side routing via `leptos_router` |
| p5.js sketches | Rewritten in Rust on `<canvas>` via `web-sys` — no JS dependency |
| Carrying over | Search, RSS, resume page, tetanes-web |
| Dropped | `/login`, `/admin` (auth had no backend), likes counter |
| Design | Static mockups of 2–3 directions reviewed before any Leptos layout work |

---

## Phase 0 — Recover what didn't survive the port

Nothing else can be verified visually until this is done.

- **42 images are missing.** `public/` has 7 icon files; posts already reference `/images/blog/*`
  and `/images/projects/*` that 404 today.
  ```
  git checkout d82ff04^ -- web/public
  git mv web/public/images web/public/resume.pdf web/public/robots.txt public/
  ```
  `fonts/`, `Roboto/`, `Yatra_One/` (~3MB) deliberately **not** recovered — deferred to Phase 5 so
  the design pass decides the typeface before we commit binary font files. `tetanes-web/` also not
  recovered: it's a prebuilt bundle from a years-old TetaNES build and should be regenerated from
  current upstream in Phase 7 rather than resurrected.
- **Two latent 404s found while doing this**, both fixed here: `public/site.webmanifest` was a stub
  with an empty `name` and icon paths (`/android-chrome-*.png`) that don't match where the icons
  actually live (`public/icons/`); and `shell()` links `/favicon.ico`, which existed only at
  `public/icons/favicon.ico`. The favicon moved to `public/` root (also where browsers implicitly
  request it).
- **26 of the old site's 27 posts are in the tree.** The missing one is at
  `git show d82ff04^:web/src/data/posts/mindyou.md`. It had a body but no metadata entry in the
  old JSON, so it was never reachable. Recover it with frontmatter or leave it retired.
- **Extract the 11 redirects** from `git show d82ff04^:web/next.config.js` (transcribed to
  `content/redirects.toml`). These are permanent redirects carrying existing SEO —
  `/articles/:year/:month/:title → /:title`, `/projects/2019/08/rustynes → /tetanes`,
  `/feed → /rss`, etc. They must be reimplemented as an Axum layer (Phase 4), not dropped.
  Order is significant: three specific `/articles/<date>/<slug>` rules must be matched before the
  generic one. Two rules (`/category/:category`, `/tag/:tag`) point at pages that never existed on
  the Next.js site either — decide in Phase 4 whether to build those listings or drop the rules.
- Delete the untracked `web/` directory once assets are recovered — it is only stale
  `node_modules`/`.next`/`coverage` output.

## Phase 1 — Dependency and feature refresh

Current lockfile vs. latest stable:

| Crate | Locked | Latest | Note |
|---|---|---|---|
| `leptos` (+ router/meta/axum) | 0.8.12 | 0.8.20 | `cargo update` — no manifest change |
| `leptos-use` | 0.16.3 | 0.19.0 | manifest bump, semver break |
| `icondata` | 0.6.0 | 0.7.0 | manifest bump, semver break |
| `comrak` | 0.47 | 0.54 | manifest bump; **moves to `[build-dependencies]`** |
| `tower-http` | 0.6.6 | 0.7.0 | manifest bump, semver break |

Enable islands. Verified against the upstream
[`islands_router` example](https://github.com/leptos-rs/leptos/tree/main/examples/islands_router):

```toml
leptos = { version = "0.8", features = ["rustls", "islands", "islands-router"] }
leptos_axum = { version = "0.8", features = ["islands-router"], optional = true }
serde = { version = "1", features = ["derive"] }   # island props are serialized

[build-dependencies]
comrak = { version = "0.54", default-features = false, features = ["syntect-fancy"] }
gray_matter = "0.3"
serde = { version = "1", features = ["derive"] }
```

`syntect-fancy` uses the pure-Rust regex engine — no oniguruma C dependency in the build graph.

Three code changes come with the feature flags:

- `src/lib.rs`: `hydrate_body(LukeWorks)` → `leptos::mount::hydrate_islands()`
- `src/lukeworks.rs`: `<HydrationScripts options islands=true islands_router=true/>`
- `shell()` renders `<App/>`; only `#[island]` subtrees ship to the browser.

## Phase 2 — Content pipeline

`content/*.toml` is mechanically-converted JSON (trailing commas, `null`, multi-line inline
tables) that no TOML parser accepts, and nothing reads it. Replace it wholesale.

**Frontmatter** — one file per post, metadata and prose together:

```markdown
---
title: "NES Emulation in Rust: Designs and Frustrations"
kind: blog            # blog | project
category: programming
image: { src: /images/blog/nes_console.webp, alt: "NES console" }
published: 2020-01-31
updated: 2020-01-31
---

**TetaNES** is an emulator for…
```

Slug = filename. `id`, `likes`, and `minutesToRead` from the old JSON are dropped —
reading time is derived at build time, the other two were unused.

**`build.rs`** walks `content/posts/`, parses frontmatter, renders markdown through comrak with
syntect highlighting, and writes a generated Rust module to `OUT_DIR`:

```rust
pub struct Post { pub slug: &'static str, pub title: &'static str, /* … */
                  pub body_html: &'static str }
pub static POSTS: &[Post] = &[ /* … */ ];
```

Two properties that matter:

- **Zero runtime I/O and zero runtime markdown parsing.** Deploy is one binary; no `content/`
  directory alongside it. `build.rs` should `cargo:rerun-if-changed=content`.
- **Gate the generated module `ssr`-only** so ~26 posts of HTML never land in the WASM bundle:
  ```rust
  #[cfg(feature = "ssr")]
  pub mod content { include!(concat!(env!("OUT_DIR"), "/content.rs")); }
  ```
  Islands that need post data (search results) get it through a server function instead.

Syntax highlighting happening at build time means no highlight.js, no client-side cost.

## Phase 3 — Islands conversion

**In islands mode `#[component]` bodies never execute in the browser.** This breaks working code
in the current tree and is the largest single rework:

- `use_theme()` (`src/hooks/use_theme.rs`) runs an `Effect` and `use_cookie` from the app root.
  As a plain component that Effect will never fire client-side and the toggle will silently do
  nothing. It must become a `#[island] ThemeToggle`.
- `RoutingProgress` in `src/lukeworks.rs` is likewise inert; the islands router handles
  transitions differently.

Also worth fixing while in here: `use_theme` reads the `sec-ch-prefers-color-scheme` request
header, but nothing sends an `Accept-CH: Sec-CH-Prefers-Color-Scheme` response header, so the
client hint is never actually sent and that branch always falls through to the `None => dark`
default. Either send `Accept-CH` from the Axum layer or delete the dead branch.

Recommended theme approach under islands: server renders `<html class="dark">` from the cookie;
a small inline script in `shell()` covers the no-cookie first visit via `matchMedia` before first
paint (avoids FOUC without any WASM); `#[island] ThemeToggle` flips the class and writes the
cookie.

Island inventory — everything else stays server-only:

| Island | Replaces |
|---|---|
| `ThemeToggle` | current root-level `use_theme` effect |
| `MobileMenu` | `header/menu/menu.tsx` |
| `Search` | `header/search/search.tsx` |
| `ShowMore` | `post/showMore.tsx` |
| `Sketch` (×9) | `components/sketch/*.ts` |
| `TetanesWeb` | `pages/tetanes-web.tsx` |

## Phase 4 — Routes and features

Register in `FlatRoutes` (currently only `/`, `/about`, `/:post` exist; the page stubs in
`src/pages/` are unrouted):

| Route | Notes |
|---|---|
| `/` | homepage — old site used `#about`/`#blog`/`#projects`/`#contact` anchors; revisit in Phase 5 |
| `/:slug` | post — flat namespace across blog and project posts, as before |
| `/resume` | from `resumeInfo.json` (`git show d82ff04^:web/src/data/resumeInfo.json`) |
| `/sketch/:name` | 9 sketches |
| `/tetanes-web` | emulator |
| `/rss` | **not** a Leptos route — a plain Axum handler returning `application/rss+xml` |
| 404 | `NotFound` is written and wired |

**Redirects** — *done*. `content/redirects.toml` is compiled to a static table by `build.rs` and
served by `redirect_middleware` (`src/server.rs`) ahead of routing; the matcher and its tests live
in `src/redirects.rs`.

Five of the twelve inherited rules were dropped, because the old site was a single page with
`#about`/`#blog`/`#projects`/`#contact` anchors and this one has real pages:

- `/about → /#about` and `/projects → /#projects` would have 301'd the new pages away and made
  them unreachable. Dropped.
- `/articles → /#blog` retargeted to `/blog`.
- `/contact → /#contact` dropped; there is no contact page to point at, so `/contact` now 404s.
  One line in `redirects.toml` retargets it if that turns out to matter.
- `/articles/category/:category` and `/:section/tag/:tag` dropped — their `/category/*` and
  `/tag/*` destinations never existed on the Next.js site either, so they 404'd then too.

`/feed → /rss` is kept and currently redirects to a 404; it starts working when RSS lands.

**Listings** — *done*. `/blog` and `/projects` render from the compiled table through the
`list_posts` server function (`src/components/post_list.rs`), the same pattern as `fetch_post`.
Markup is deliberately plain pending Phase 5.

**Search** — the upstream islands_router example does exactly this shape: a `<form method="GET">`
whose input is an island, results fetched by `#[server]` fn over the build-time index and
rendered server-side. Degrades to a working form with WASM disabled.

**RSS** — generate from the same `POSTS` index; add `<link rel="alternate">` in `shell()`.

## Phase 5 — Design directions

Deliverable before any Leptos layout work: 2–3 static HTML mockups (home, post, listing) to
compare side by side. Constraints carried in from the current tree:

- Tailwind 4 — theme tokens live in `@theme` in `style/tailwind.css`, not `tailwind.config.js`.
- The existing token set (`--color-primary`, `--color-action-1`, …) is already partly orphaned:
  `src/lukeworks.rs` references `text-brand-fg2`, which no longer exists. The redesign should
  settle the palette and then a single pass can fix every stale class.
- Dark mode is a `.dark` class variant and must stay server-decidable (see Phase 3).
- `style/main.css` is empty but is the `style-file` cargo-leptos expects — keep it or repoint.

## Phase 6 — Sketches in Rust

Nine sketches, currently p5.js: `asteroids`, `fireworks`, `fluid-simulation`, `fourier`,
`lorenz-attractor`, `matrix`, `maze-astar`, `pong`, `raycasting-2d`
(`git show d82ff04^:web/src/components/sketch/<name>.ts`).

Build a shared `sketch` runtime first — `requestAnimationFrame` loop, canvas resize/DPR handling,
pointer and key input, start/stop on visibility — then port each sketch onto it. `fluid-simulation`
and `fourier` are the substantial ones; `matrix` and `pong` are good first ports to validate the
runtime.

This is where WASM size becomes real, and where the `--split` flag already in the `justfile`
earns its place: mark each sketch `#[lazy]` so its chunk is fetched only when that page is
visited. Note `--split` has had rough edges upstream
([leptos#4322](https://github.com/leptos-rs/leptos/issues/4322)) — verify a split build serves
correctly before depending on it, and be ready to drop the flag until sketches actually land.

## Phase 7 — Deploy

`.github/workflows/ci.yml` runs lint → audit → test → build, and its deploy step is a bare
`# TODO`. Needs deciding: the repo still has a `.dockerignore` from an earlier container setup,
and `[package.metadata.leptos]` has no `site-addr`/`env` configured. Release builds must set
`LEPTOS_HASH_FILES=true` — `RootStylesheet` switches on it.

---

## Sequencing

Phases 0–2 are the critical path and unblock everything else; 3–4 turn it into a working site;
5 is gated on your review of the mockups; 6 is the long tail and independent of the rest.

The redirect table now has the table-driven tests it deserved (`src/redirects.rs`). Frontmatter
parsing still has none — `build.rs` failures there remain confusing.

## Open: is `--split` safe?

`just dev`, `just run` and `just build` all pass `--split`. During Phase 4 a split build threw
`RuntimeError: function signature mismatch` out of a wasm-bindgen closure shim on every nav click
and client-side routing failed outright; the same page worked with a plain `cargo leptos watch`.

That test was **confounded** — dev builds were serving `.wasm` with `max-age=30d, immutable` at the
time (see the cache-control note in `CLAUDE.md`), so the browser may simply have been pairing a
stale module with fresh glue. The caching bug is fixed; the split retest was never redone because
browser automation dropped out mid-session. Redo it before trusting `--split`:

1. `cargo leptos watch --split`, load a post, click a nav link.
2. Client-side nav should change the URL without a full page load, and the console should be clean.

If it still fails, drop `--split` from the `justfile` until Phase 6 actually needs lazy chunks —
[leptos#4322](https://github.com/leptos-rs/leptos/issues/4322) is the known rough edge.
