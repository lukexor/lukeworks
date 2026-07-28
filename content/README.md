# content/

## `posts/*.md`

One file per post. The filename (without `.md`) is the slug and the URL:
`posts/tetanes-part-2.md` serves at `/tetanes-part-2`. Blog and project posts
share one flat namespace, as they did on the Next.js site.

`build.rs` parses the frontmatter and renders the body to HTML at compile time.
Adding a post means adding one file — there is no index to update.

```yaml
---
title: "NES Emulation in Rust: Designs and Frustrations"   # required
kind: blog                                                 # required: blog | project
category: programming                                      # optional, lowercase
image:                                                     # optional
  src: /images/blog/nes_console.webp
  alt: "Nintendo Entertainment System console"
website: https://github.com/lukexor/pix-engine             # optional, projects
published: 2020-01-31T21:19:14Z                            # omit for drafts
started: 2019-09-19T07:11:06Z                              # optional, projects
completed: 2020-01-31T21:19:14Z                            # optional, projects
updated: 2019-09-19T07:12:30Z                              # optional
draft: true                                                # optional, default false
---

Body markdown starts here.
```

Notes:

- **Timestamps are RFC 3339 and treated as UTC.** The originals came out of a
  MySQL dump with no timezone attached, so UTC is an assumption, not a fact.
  Full timestamps rather than dates because four pairs of posts share a
  publication date and the time-of-day is what orders them.
- **Reading time is derived** (~200 wpm) rather than stored. The old
  `minutesToRead` field is gone.
- **Drafts** — a post with `draft: true` or no `published` is excluded from
  listings but still compiled. `mindyou.md` is the only one: it has a body but
  never had a metadata entry in the old site's JSON, so it was never reachable.
- Dropped from the old JSON: `id` and `likes` (unused) and `tags` (empty on
  every single entry). Re-add `tags` as a real field if tag pages get built.
- `image` and `thumbnail` were collapsed into `image`. They were identical
  wherever both were set, and `image` was null in exactly the posts where it
  differed — so the distinction carried no information.

Post bodies may contain raw HTML (several embed `<iframe>` or `<img>` grids); it
is passed through, which is safe because this markdown is ours and is rendered
at build time, never from user input.

## `portfolio.toml`

Site copy — homepage headings, about and contact text, error-page strings. Valid
TOML, unlike the two post-metadata files it used to sit beside, which were an
unparseable JSON-to-TOML conversion and have been replaced by the frontmatter
above.

## `redirects.toml`

The 11 permanent redirects inherited from the Next.js site. Order is
significant; see the comments in the file.
