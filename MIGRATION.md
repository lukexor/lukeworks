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
| `/tetanes-web` | emulator — see the note below |
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

**tetanes-web** on the live site frames `../tetanes`'s generated `index.html`, so it reads as a
separate site sitting inside this one: its own chrome, its own theme, none of the page's. The page
should host the emulator directly and wear the site's styling like any other project page. That
likely means changing what `../tetanes` exposes, to a mountable canvas and controls rather than a
whole document.

## Phase 5 — Design directions

Deliverable before any Leptos layout work: 2–3 static HTML mockups (home, post, listing) to
compare side by side. Constraints carried in from the current tree:

- Tailwind 4 — theme tokens live in `@theme` in `style/tailwind.css`, not `tailwind.config.js`.
- The existing token set (`--color-primary`, `--color-action-1`, …) is already partly orphaned:
  `src/lukeworks.rs` references `text-brand-fg2`, which no longer exists. The redesign should
  settle the palette and then a single pass can fix every stale class.
- Dark mode is a `.dark` class variant and must stay server-decidable (see Phase 3).
- `style/main.css` is empty but is the `style-file` cargo-leptos expects — keep it or repoint.

Polish wanted here: type the hero's "Hi, I'm Luke" out like a terminal, with the trailing `_`
following the cursor as characters land. The subtitle either fades in after or types too, to be
decided when it can be seen side by side.

## Phase 6 — Sketches in Rust

Nine sketches, currently p5.js: `asteroids`, `fireworks`, `fluid-simulation`, `fourier`,
`lorenz-attractor`, `matrix`, `maze-astar`, `pong`, `raycasting-2d`
(`git show d82ff04^:web/src/components/sketch/<name>.ts`).

Build a shared `sketch` runtime first — `requestAnimationFrame` loop, canvas resize/DPR handling,
pointer and key input, start/stop on visibility — then port each sketch onto it. `fluid-simulation`
and `fourier` are the substantial ones; `matrix` and `pong` are good first ports to validate the
runtime.

This is where WASM size becomes real. Every command already passes `--split`, so marking each
sketch `#[lazy]` is enough to have its chunk fetched only when that page is visited. Watch for the
rough edges upstream ([leptos#4322](https://github.com/leptos-rs/leptos/issues/4322)): every
command depends on the flag now, so a regression there is a broken site rather than a larger
bundle.

## Phase 7 — Deploy

Two jobs to do once the tree settles and before the first deploy, both of which move a lot at once
and want a quiet moment:

- **Dependency refresh.** `cargo update`, then `just audit` and both clippy targets.
- **Leptos 0.9.0-beta.** It carries performance work and bug fixes over the 0.8 line this is built
  on. `leptos`, `leptos_router`, `leptos_meta` and `leptos_axum` move together, and cargo-leptos
  has to be current with them. Worth trying on a branch: the two places most likely to break are
  the `#[lazy_route]` impls and `hydrate_lazy`, both of which are recent API.

Target is Fly.io: one `shared-cpu-1x` machine at 512MB, ~$3.30/mo, suspended while idle. The
alternative considered was a €4 VPS, which is cheaper and hands you TLS renewal and OS patching.

In the tree: `Dockerfile` builds the release and ships the binary, `hash.txt` and `target/site`
on a slim Debian. `fly.toml` sets the runtime configuration. `ci.yml`'s `deploy` job runs
`flyctl deploy --remote-only` on a green push to `main`.

Configuration is all environment, because `[package.metadata.leptos]` is a build-time table the
binary never reads. The four that matter: `LEPTOS_SITE_ROOT=site`, `LEPTOS_SITE_ADDR=0.0.0.0:8080`
(the default binds 127.0.0.1, which answers nothing from outside a container), `LEPTOS_ENV=PROD`,
and `LEPTOS_HASH_FILES=true`, which `RootStylesheet` and the hashed-asset cache headers both
switch on. `hash.txt` sits beside the binary, and the binary resolves both it and the site root
against the working directory.

Left to do, none of which can be done from here:

1. `fly auth login`, then `fly apps create lukeworks` (or `fly launch --no-deploy`, which will
   want to overwrite `fly.toml`). Check that `primary_region` names the region you want, since
   it is set to `sea`.
2. `fly tokens create deploy -a lukeworks`, saved as the `FLY_API_TOKEN` repository secret.
3. `fly certs add lukeworks.tech` and the `www` variant, then point DNS. The domain is
   registered with Vercel, and registration does not have to move: the Domains panel takes
   custom nameservers, or the A/AAAA records can point at Fly directly. Transferring out needs an
   auth code from that same panel, and ICANN blocks a transfer within 60 days of registration.

---

## Sequencing

Phases 0–2 are the critical path and unblock everything else; 3–4 turn it into a working site;
5 is gated on your review of the mockups; 6 is the long tail and independent of the rest.

The redirect table now has the table-driven tests it deserved (`src/redirects.rs`). Frontmatter
parsing still has none — `build.rs` failures there remain confusing.

## Answered: `--split` is mandatory

The routes marked `#[lazy_route]` compile to a module import of `__wasm_split_placeholder__` that
only a split build rewrites, so `just dev`, `just run`, `just build` and the `Dockerfile` all pass
the flag. Dropping it anywhere leaves that page importing a specifier the browser cannot resolve,
and the whole app stays inert. See the "Lazy routes" section of `CLAUDE.md`.

The `RuntimeError: function signature mismatch` seen during Phase 4 does not reproduce, and neither
does a `Could not rename target/site/pkg/lukeworks_bg.wasm` failure on rebuild: a watcher under
`--split` survives repeated edits, including one with a concurrent `cargo nextest` against the same
target directory. What remains untested is a split *release* bundle driven from a browser, which
means loading a post and clicking a nav link with the console open.
