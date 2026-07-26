# Next.js → Leptos migration plan

Target architecture: **Leptos 0.8 SSR + islands**, content compiled to HTML at build time,
single self-contained binary.

The authoritative spec for "what the old site did" is git, not the working tree. The Next.js
source was deleted in `d82ff04` (*refactor: initial leptos version*); read it with:

```
git show d82ff04^:web/src/pages/index.tsx
git ls-tree -r --name-only d82ff04^ -- web/src
```

---

## Decisions

| Area | Decision |
|---|---|
| Post metadata | YAML frontmatter in each `content/posts/*.md`; delete `content/*.toml` |
| Rendering | Markdown → HTML at **build time** (`build.rs` + comrak), embedded in the server binary |
| Interactivity | Islands (`#[island]`), everything else server-rendered only |
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
- **`first-post.md` was dropped** (26 of 27 posts came across). Recover or intentionally retire it.
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

**Redirects** — an `axum::middleware` layer translating the 12 rules from `next.config.js`.
Worth a table-driven test; these are the one thing that breaks silently and costs real traffic.

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

There are no tests in the tree. Two places deserve them as they're built: the redirect table
(silent SEO regressions) and frontmatter parsing (`build.rs` failures are confusing).
