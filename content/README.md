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
description: A cycle-accurate NES emulator.                # optional, derived if absent
featured: true                                             # optional, projects, default false
category: programming                                      # optional, lowercase
series: TetaNES                                            # optional, groups posts
part: 2                                                    # optional, omit on an intro
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
- **Reading time is derived** (~200 wpm) rather than stored.
- **`description` is the card blurb**, and falls back to the first paragraph of
  the body, flattened and cut to 165 characters. The fallback skips the block
  elements a post may open with (a heading, a fenced block, a blockquote, a
  link-reference definition, raw HTML), so `matrix.md` does not lead with the
  `<iframe>` that embeds its sketch. Write one by hand where the first paragraph
  reads badly out of context, or where flattening loses something: an `A*` comes
  out as `A`, since nothing distinguishes that asterisk from a closing emphasis
  marker.
- **`featured` lifts a project onto the homepage.** Three are marked. With none
  marked the homepage falls back to the three most recent, so the section is
  never empty. It does nothing on a blog post.
- **Series.** `series` is the display name and repeats verbatim in every post
  that belongs to it. `part` orders them. A post carrying a `series` but no
  `part` is the introduction and sorts ahead of part 1. `build.rs` rejects a
  `part` without a `series`, two posts claiming the same part, and two spellings
  of one series name, since a typo would otherwise split a series in two.
  Two exist: `Lost and Found` (intro plus five parts) and `TetaNES` (two parts).
- **Drafts.** A post with `draft: true` or no `published` is excluded from
  listings but still compiled. There are none right now.
- There is no `tags` field. Add one here and in `build.rs` if tag pages get
  built.

Post bodies may contain raw HTML (several embed `<iframe>` or `<img>` grids); it
is passed through, which is safe because this markdown is ours and is rendered
at build time, never from user input.

## `portfolio.toml`

Site copy: homepage headings, about and contact text, error-page strings.
Nothing reads it yet.

## `redirects.toml`

The 6 permanent redirects inherited from the Next.js site. Order is
significant; see the comments in the file.
