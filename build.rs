//! Compiles `content/posts/*.md` into a static `POSTS` table,
//! `content/redirects.toml` into a static `REDIRECTS` table, and the head of
//! `src/lukeworks.rs` into the `CODE_BACKDROP` the hero sits on.
//!
//! Frontmatter is parsed and markdown is rendered to HTML here, at build time,
//! so the server does no I/O and no markdown parsing at runtime and the binary
//! is self-contained — there is no `content/` directory to deploy alongside it.
//!
//! The generated post table is compiled only into the `ssr` build. See the note
//! on `POSTS` below. `REDIRECTS` is small and is emitted for both targets.

use comrak::{
    Options, markdown_to_html_with_plugins, options::Plugins, plugins::syntect::SyntectAdapter,
};
use gray_matter::{Matter, engine::YAML};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, btree_map::Entry},
    env,
    fmt::Write as _,
    fs,
    path::Path,
};

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    kind: String,
    /// One-line blurb for a card. Falls back to [`excerpt`] over the body.
    description: Option<String>,
    /// Lifts a project onto the homepage.
    #[serde(default)]
    featured: bool,
    /// What a project was built with, for the card eyebrow.
    #[serde(default)]
    technologies: Vec<String>,
    category: Option<String>,
    series: Option<String>,
    part: Option<usize>,
    image: Option<Image>,
    website: Option<String>,
    published: Option<String>,
    started: Option<String>,
    completed: Option<String>,
    updated: Option<String>,
    #[serde(default)]
    draft: bool,
}

#[derive(Deserialize)]
struct Image {
    src: String,
    alt: Option<String>,
}

/// One parsed post, between reading the file and generating the table.
struct Parsed {
    slug: String,
    front: FrontMatter,
    body_html: String,
    description: String,
    reading_minutes: usize,
}

fn main() {
    println!("cargo:rerun-if-changed=content/posts");
    println!("cargo:rerun-if-changed=content/redirects.toml");
    println!("cargo:rerun-if-changed=src/lukeworks.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    write_redirects(Path::new(&out_dir));

    let dir = Path::new("content/posts");
    let matter = Matter::<YAML>::new();

    // `None` emits CSS classes rather than inline styles, so code blocks can
    // follow the light/dark theme. The rules behind those classes live under
    // `pre.syntax-highlighting` in style/tailwind.css.
    let adapter = SyntectAdapter::new(None);
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let mut options = Options::default();
    options.extension.alerts = true;
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    // Enables heading anchors, so posts can be deep-linked.
    options.extension.header_id_prefix = Some(String::new());
    // Posts contain hand-written HTML (e.g. the <img> grids in tetanes.md),
    // which is dropped unless raw HTML is allowed through. Safe here: the
    // markdown is ours and is rendered at build time, never from user input.
    options.render.r#unsafe = true;

    write_backdrop(Path::new(&out_dir));

    let mut posts = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    entries.sort();

    for path in entries {
        let slug = path.file_stem().unwrap().to_string_lossy().into_owned();
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));

        let parsed = matter
            .parse::<FrontMatter>(&raw)
            .unwrap_or_else(|e| panic!("{}: bad frontmatter: {e}", path.display()));
        let fm = parsed
            .data
            .unwrap_or_else(|| panic!("{}: missing frontmatter", path.display()));

        if fm.kind != "blog" && fm.kind != "project" {
            panic!("{}: kind must be \"blog\" or \"project\"", path.display());
        }

        if fm.part.is_some() && fm.series.is_none() {
            panic!("{}: `part` needs a `series` to be part of", path.display());
        }

        let body_html = markdown_to_html_with_plugins(&parsed.content, &options, &plugins);
        // ~200 wpm, the usual reading-speed approximation.
        let words = parsed.content.split_whitespace().count();
        let reading_minutes = words.div_ceil(200).max(1);
        let description = fm
            .description
            .clone()
            .unwrap_or_else(|| excerpt(&parsed.content));

        posts.push(Parsed {
            slug,
            front: fm,
            body_html,
            description,
            reading_minutes,
        });
    }

    check_series(&posts);

    // Newest first. Sorting here means the runtime never parses a date to order
    // a listing. Undated drafts sort last.
    posts.sort_by(|a, b| {
        b.front
            .published
            .as_deref()
            .unwrap_or("")
            .cmp(a.front.published.as_deref().unwrap_or(""))
    });

    let mut out = String::new();
    out.push_str(
        r#"// @generated by build.rs from content/posts/*.md — do not edit.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Blog,
    Project,
}

#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub src: &'static str,
    pub alt: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub kind: Kind,
    /// One-line blurb, from the frontmatter or derived from the body.
    pub description: &'static str,
    /// Set on the projects the homepage leads with.
    pub featured: bool,
    /// What a project was built with, in the order it should read.
    pub technologies: &'static [&'static str],
    pub category: Option<&'static str>,
    /// Display name of the series this post belongs to.
    pub series: Option<&'static str>,
    /// Position within [`Post::series`]. `None` marks the series introduction,
    /// which sorts ahead of every numbered part.
    pub part: Option<usize>,
    pub image: Option<Image>,
    pub website: Option<&'static str>,
    /// RFC 3339, e.g. "2020-01-31T21:19:14Z".
    pub published: Option<&'static str>,
    pub started: Option<&'static str>,
    pub completed: Option<&'static str>,
    pub updated: Option<&'static str>,
    pub draft: bool,
    pub reading_minutes: usize,
    pub body_html: &'static str,
}

"#,
    );

    // Only the `ssr` build gets the post bodies. Under `hydrate` this is an
    // empty slice, which keeps ~26 posts of rendered HTML out of the WASM
    // bundle while letting server-only components still typecheck for wasm32.
    // Component bodies re-run in the browser during hydration, so a component
    // reading this directly finds it empty there. Reach it through a server
    // function instead.
    out.push_str("#[cfg(not(feature = \"ssr\"))]\npub static POSTS: &[Post] = &[];\n\n");
    out.push_str("#[cfg(feature = \"ssr\")]\npub static POSTS: &[Post] = &[\n");
    for Parsed {
        slug,
        front: fm,
        body_html,
        description,
        reading_minutes,
    } in &posts
    {
        let kind = if fm.kind == "blog" {
            "Kind::Blog"
        } else {
            "Kind::Project"
        };
        writeln!(out, "    Post {{").unwrap();
        writeln!(out, "        slug: {},", lit(slug)).unwrap();
        writeln!(out, "        title: {},", lit(&fm.title)).unwrap();
        writeln!(out, "        kind: {kind},").unwrap();
        writeln!(out, "        description: {},", lit(description)).unwrap();
        writeln!(out, "        featured: {},", fm.featured).unwrap();
        let technologies: Vec<_> = fm.technologies.iter().map(|it| lit(it)).collect();
        writeln!(out, "        technologies: &[{}],", technologies.join(", ")).unwrap();
        writeln!(out, "        category: {},", opt(fm.category.as_deref())).unwrap();
        writeln!(out, "        series: {},", opt(fm.series.as_deref())).unwrap();
        match fm.part {
            Some(part) => writeln!(out, "        part: Some({part}),").unwrap(),
            None => writeln!(out, "        part: None,").unwrap(),
        }
        match &fm.image {
            Some(img) => writeln!(
                out,
                "        image: Some(Image {{ src: {}, alt: {} }}),",
                lit(&img.src),
                lit(img.alt.as_deref().unwrap_or(""))
            )
            .unwrap(),
            None => writeln!(out, "        image: None,").unwrap(),
        }
        writeln!(out, "        website: {},", opt(fm.website.as_deref())).unwrap();
        writeln!(out, "        published: {},", opt(fm.published.as_deref())).unwrap();
        writeln!(out, "        started: {},", opt(fm.started.as_deref())).unwrap();
        writeln!(out, "        completed: {},", opt(fm.completed.as_deref())).unwrap();
        writeln!(out, "        updated: {},", opt(fm.updated.as_deref())).unwrap();
        writeln!(out, "        draft: {},", fm.draft).unwrap();
        writeln!(out, "        reading_minutes: {reading_minutes},").unwrap();
        writeln!(out, "        body_html: {},", lit(body_html)).unwrap();
        writeln!(out, "    }},").unwrap();
    }
    out.push_str("];\n");

    let dest = Path::new(&out_dir).join("content.rs");
    fs::write(&dest, out).unwrap_or_else(|e| panic!("cannot write {}: {e}", dest.display()));
}

/// Lines of `src/lukeworks.rs` the hero backdrop shows.
///
/// The widest layout flows the backdrop into three 24rem columns of a ~450px
/// band, which a 17px line fills with 69, so this leaves a few spare. The rest
/// is clipped by `.code-backdrop`, and every line adds ~300 bytes to both
/// targets.
const BACKDROP_LINES: usize = 72;

/// Compile the head of `src/lukeworks.rs` into the hero's backdrop.
///
/// The homepage sits on a wall of blurred code, and the code is the app root
/// itself. Plain escaped text, in one colour the stylesheet picks per theme,
/// rather than the syntect pass the blog's fenced blocks get: highlighting 72
/// lines emitted 511 spans, 78% of every element on the page, for detail that a
/// 2px blur takes straight back out. The text alone is 3.9KB against 27KB.
///
/// Emitted for both targets, unlike `POSTS`. See `src/backdrop.rs` for why.
///
/// **These lines are published verbatim,** in the page source and in the WASM
/// bundle, blur or no blur. Point this at a file chosen for the backdrop if
/// `src/lukeworks.rs` ever grows something that should not be read off the
/// homepage, such as an unlisted route.
fn write_backdrop(out_dir: &Path) {
    let path = Path::new("src/lukeworks.rs");
    let source =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));

    // The import block is both the longest lines in the file (98 columns, wider
    // than a backdrop column) and the least interesting to look at.
    let mut lines = source.lines();
    let mut head: Vec<&str> = Vec::with_capacity(BACKDROP_LINES);
    while head.len() < BACKDROP_LINES {
        let Some(line) = lines.next() else { break };
        if line.starts_with("use ") {
            if !line.ends_with(';') {
                lines.by_ref().find(|line| line.trim_end() == "};");
            }
            continue;
        }
        head.push(line);
    }
    // The window is source, not markup, and it renders inside a <pre> through
    // `inner_html`. Escaping keeps a `<` in it text, so the generics and the
    // `view!` bodies in this file stay source rather than reaching the homepage
    // as live markup.
    let mut text = String::with_capacity(4096);
    for line in &head {
        for c in line.chars() {
            match c {
                '<' => text.push_str("&lt;"),
                '>' => text.push_str("&gt;"),
                '&' => text.push_str("&amp;"),
                _ => text.push(c),
            }
        }
        text.push('\n');
    }
    let html = format!("<pre>{text}</pre>");

    let mut out = String::from(
        "// @generated by build.rs from src/lukeworks.rs — do not edit.\n\n\
         /// The site's own source, for the homepage hero to sit on.\n\
         ///\n\
         /// Blurred, and veiled by the hero's scrim, so it reads as texture.\n",
    );
    writeln!(out, "pub const CODE_BACKDROP: &str = {};", lit(&html)).unwrap();

    let dest = out_dir.join("backdrop.rs");
    fs::write(&dest, out).unwrap_or_else(|e| panic!("cannot write {}: {e}", dest.display()));
}

/// Longest excerpt [`excerpt`] will return, in characters.
const EXCERPT_CHARS: usize = 165;

/// Derive a one-line blurb from the body of a post.
///
/// The fallback for a post with no `description:` in its frontmatter, so all 26
/// existing posts get a card blurb without being edited. Takes the first
/// paragraph of prose and flattens the inline markdown in it.
///
/// Skipping matters more than the flattening does. A body may open with a
/// heading, a fenced code block, a blockquote, a link-reference definition, or
/// raw HTML: `matrix.md` starts with the `<iframe>` that embeds its sketch, and
/// lifting that as the blurb would put markup on the homepage.
fn excerpt(body: &str) -> String {
    let mut paragraph = String::new();
    let mut lines = body.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();

        if line.starts_with("```") || line.starts_with("~~~") {
            let fence = &line[..3];
            for line in lines.by_ref() {
                if line.trim_start().starts_with(fence) {
                    break;
                }
            }
            continue;
        }

        // A bullet or heading marker only counts as one when whitespace follows
        // it. `**Haskelltaire** is a…` opens a paragraph, not a list, and
        // treating that lone `*` as a bullet skipped every body that leads with
        // a bold word.
        let mut chars = line.chars();
        let first = chars.next();
        let second = chars.next();
        let spaced = matches!(second, None | Some(' ' | '\t'));
        let bullet = matches!(first, Some('-' | '*' | '+')) && spaced;
        let heading = first == Some('#') && (spaced || second == Some('#'));
        // `1. ` opens an ordered list. The marker is the leading run of digits
        // and nothing else, so a paragraph opening `3.14 is the ratio…` stays
        // prose rather than being read as list markup and skipped.
        let digits = line.len() - line.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        let ordered = digits > 0
            && matches!(line[digits..].chars().next(), Some('.' | ')'))
            && matches!(line[digits..].chars().nth(1), None | Some(' ' | '\t'));

        // A markdown block element ends at the first blank line, and so does an
        // HTML block. Both are skipped whole.
        // `[` opens a link-reference definition, `![` a standalone image. Alt
        // text is a caption, not a blurb, so the image block goes whole.
        let skipped = line.is_empty()
            || bullet
            || heading
            || ordered
            || line.starts_with("![")
            || line.starts_with(['>', '|', '<', '[']);
        if skipped {
            if !line.is_empty() {
                for line in lines.by_ref() {
                    if line.trim().is_empty() {
                        break;
                    }
                }
            }
            continue;
        }

        // The first line of real prose. A paragraph runs to the next blank.
        paragraph.push_str(line);
        for line in lines.by_ref() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            paragraph.push(' ');
            paragraph.push_str(line);
        }
        break;
    }

    truncate(&flatten_markdown(&paragraph), EXCERPT_CHARS)
}

/// Reduce the inline markdown in one paragraph to its text.
///
/// Handles what post bodies actually use: emphasis, inline code, and both link
/// forms, including the `[text][ref]` shortcut whose target sits at the bottom
/// of the file. Anything else is left as written rather than guessed at.
fn flatten_markdown(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '*' | '_' | '`' => {}
            '\\' => out.extend(chars.next()),
            // The `!` of an inline image. Dropping it leaves the `[` arm below
            // to keep the alt text, which is the only prose in an image.
            '!' if chars.peek() == Some(&'[') => {}
            '[' => {
                for c in chars.by_ref() {
                    if c == ']' {
                        break;
                    }
                    out.push(c);
                }
                // The target that follows, in either bracket style.
                let closer = match chars.peek() {
                    Some('(') => Some(')'),
                    Some('[') => Some(']'),
                    _ => None,
                };
                if let Some(closer) = closer {
                    chars.next();
                    for c in chars.by_ref() {
                        if c == closer {
                            break;
                        }
                    }
                }
            }
            _ => out.push(c),
        }
    }

    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Cut to `limit` characters on a word boundary, adding an ellipsis.
fn truncate(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_owned();
    }

    let head: String = text.chars().take(limit).collect();
    let cut = head.rfind(' ').unwrap_or(head.len());
    format!("{}…", head[..cut].trim_end_matches([',', ';', ':', '.']))
}

struct Rule {
    from: String,
    to: String,
}

/// Reject the two ways a series can be miswritten across files.
///
/// Series membership is a plain string repeated in every post, so a divergent
/// spelling does not fail anything by itself: it quietly produces a second
/// series with one post in it. Normalising away case, spaces, dashes and
/// underscores catches the near misses (`TetaNES` against `tetanes`, `Lost and
/// Found` against `lost-and-found`). A genuine misspelling of the letters
/// themselves still slips through, since nothing here can tell it apart from a
/// second series that really does have one post so far. Rejecting duplicate
/// parts is the other half, and keeps the reading order total.
fn check_series(posts: &[Parsed]) {
    let key = |name: &str| name.to_lowercase().replace([' ', '-', '_'], "");

    let mut canonical: BTreeMap<String, (&str, &str)> = BTreeMap::new();
    let mut parts: BTreeMap<(String, usize), &str> = BTreeMap::new();

    for Parsed {
        slug, front: fm, ..
    } in posts
    {
        let Some(series) = fm.series.as_deref() else {
            continue;
        };

        match canonical.entry(key(series)) {
            Entry::Vacant(e) => {
                e.insert((series, slug));
            }
            Entry::Occupied(e) => {
                let (seen_name, seen_slug) = *e.get();
                assert!(
                    seen_name == series,
                    "series name disagrees between posts: {seen_slug} says {seen_name:?}, \
                     {slug} says {series:?}. Pick one spelling."
                );
            }
        }

        if let Some(part) = fm.part
            && let Some(other) = parts.insert((key(series), part), slug)
        {
            panic!("{series:?} part {part} is claimed by both {other} and {slug}");
        }
    }
}

/// Read the `[[redirect]]` tables out of `content/redirects.toml`.
///
/// A hand-rolled reader rather than the `toml` crate — see the note in
/// `[build-dependencies]` for why that dependency had to go. This accepts only
/// the shape the file actually uses (`[[redirect]]` headers plus `from`/`to`
/// double-quoted strings) and panics on anything else, so a typo fails the
/// build instead of silently dropping a rule.
fn read_rules(path: &Path, raw: &str) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut from: Option<String> = None;
    let mut to: Option<String> = None;

    // Rules are only pushed when the *next* header (or EOF) is reached, so a
    // half-written rule is caught rather than silently merged with the next.
    let mut flush = |from: &mut Option<String>, to: &mut Option<String>, line: usize| match (
        from.take(),
        to.take(),
    ) {
        (Some(from), Some(to)) => rules.push(Rule { from, to }),
        (None, None) => {}
        _ => panic!(
            "{}:{line}: a [[redirect]] needs both `from` and `to`",
            path.display()
        ),
    };

    for (index, line) in raw.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line == "[[redirect]]" {
            flush(&mut from, &mut to, line_number);
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            panic!(
                "{}:{line_number}: expected `key = \"value\"`",
                path.display()
            );
        };
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .unwrap_or_else(|| {
                panic!(
                    "{}:{line_number}: value must be a double-quoted string",
                    path.display()
                )
            });

        let slot = match key.trim() {
            "from" => &mut from,
            "to" => &mut to,
            other => panic!(
                "{}:{line_number}: unknown key `{other}` (expected `from` or `to`)",
                path.display()
            ),
        };
        assert!(
            slot.replace(value.to_owned()).is_none(),
            "{}:{line_number}: `{}` set twice in one [[redirect]]",
            path.display(),
            key.trim()
        );
    }
    flush(&mut from, &mut to, raw.lines().count());

    assert!(
        !rules.is_empty(),
        "{}: no [[redirect]] rules found",
        path.display()
    );
    rules
}

/// Compile `content/redirects.toml` into a `REDIRECTS` table.
///
/// `from` is pre-split into segments here so the runtime matcher only compares
/// already-parsed literals against the request path. The declaration order in
/// the file is preserved and significant: the matcher takes the first hit, and
/// the specific `/articles/<date>/<slug>` rules have to beat the generic one.
fn write_redirects(out_dir: &Path) {
    let path = Path::new("content/redirects.toml");
    let raw =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let rules = read_rules(path, &raw);

    let mut out = String::from(
        r#"// @generated by build.rs from content/redirects.toml — do not edit.

/// One path segment of a `from` pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    /// Matches this exact text.
    Literal(&'static str),
    /// Matches any single segment, capturing it under this name.
    Param(&'static str),
}

#[derive(Debug, Clone, Copy)]
pub struct Redirect {
    pub from: &'static [Segment],
    /// Destination path. A `:name` here is replaced by the captured segment.
    pub to: &'static str,
}

pub static REDIRECTS: &[Redirect] = &[
"#,
    );

    for rule in &rules {
        assert!(
            rule.from.starts_with('/') && rule.to.starts_with('/'),
            "{}: `{}` -> `{}`: both sides must be absolute paths",
            path.display(),
            rule.from,
            rule.to
        );

        let segments: Vec<String> = rule
            .from
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| match s.strip_prefix(':') {
                Some(name) => format!("Segment::Param({})", lit(name)),
                None => format!("Segment::Literal({})", lit(s)),
            })
            .collect();

        // A `:name` in `to` has to have been captured by `from`, or the rule
        // would emit a Location header with a literal ":name" in it.
        for want in rule.to.split('/').filter_map(|s| s.strip_prefix(':')) {
            assert!(
                rule.from
                    .split('/')
                    .any(|s| s.strip_prefix(':') == Some(want)),
                "{}: `{}` -> `{}`: `:{want}` is not captured by the `from` pattern",
                path.display(),
                rule.from,
                rule.to
            );
        }

        writeln!(
            out,
            "    Redirect {{ from: &[{}], to: {} }},",
            segments.join(", "),
            lit(&rule.to)
        )
        .unwrap();
    }
    out.push_str("];\n");

    let dest = out_dir.join("redirects.rs");
    fs::write(&dest, out).unwrap_or_else(|e| panic!("cannot write {}: {e}", dest.display()));
}

/// Emit a Rust string literal. Post bodies contain `"`, `\`, `#` and `{}` in
/// arbitrary combinations, so escape rather than trying to pick a raw-string
/// hash count.
fn lit(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn opt(s: Option<&str>) -> String {
    match s.filter(|v| !v.is_empty()) {
        Some(v) => format!("Some({})", lit(v)),
        None => "None".into(),
    }
}
