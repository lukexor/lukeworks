//! A single blog or project post.

use crate::{
    components::{
        icons::{ArrowIcon, BackArrowIcon},
        post_list::category_href,
    },
    lukeworks::ROUTES,
    pages::not_found::NotFound,
};
use leptos::{
    either::{Either, EitherOf3},
    prelude::*,
};
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

#[derive(Params, Debug, Clone, PartialEq)]
#[must_use]
struct PostParams {
    post: Option<String>,
}

/// One `<h2>` in the body, for the table of contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    /// Fragment id comrak generated from the heading text.
    pub id: String,
    /// The heading's own text.
    pub text: String,
}

/// A sibling post in the same series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesEntry {
    /// Slug to link to.
    pub slug: String,
    /// The sibling's title.
    pub title: String,
    /// Position in the series. `None` marks the introduction.
    pub part: Option<usize>,
}

/// The post either side of this one, in publication order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neighbour {
    /// Slug to link to.
    pub slug: String,
    /// The neighbour's title.
    pub title: String,
}

/// A post as sent to the browser.
///
/// Owned rather than `&'static str` because this crosses the network boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostView {
    /// Filename without extension, which is also the URL.
    pub slug: String,
    /// Post title.
    pub title: String,
    /// Body, already rendered to HTML at build time.
    pub body_html: String,
    /// Derived at build time from the word count.
    pub reading_minutes: usize,
    /// RFC 3339 publication timestamp.
    pub published: Option<String>,
    /// Lowercase category from the frontmatter.
    pub category: Option<String>,
    /// What a project was built with. Empty on a blog post.
    pub technologies: Vec<String>,
    /// A project shows no reading time, since every one lands at a minute or
    /// two and the number says nothing.
    pub show_reading_time: bool,
    /// Header image path.
    pub image_src: Option<String>,
    /// Alt text for [`PostView::image_src`].
    pub image_alt: Option<String>,
    /// Display name of the series this post belongs to.
    pub series: Option<String>,
    /// Every `<h2>` in the body, for the table of contents.
    pub headings: Vec<Heading>,
    /// The whole series in reading order, this post included.
    pub siblings: Vec<SeriesEntry>,
    /// The next older post of the same kind.
    pub previous: Option<Neighbour>,
    /// The next newer post of the same kind.
    pub next: Option<Neighbour>,
}

/// Pull the `<h2>` ids and text back out of the rendered body.
///
/// Comrak writes each heading as `<h2 id="…">Text<a … data-heading-content="Text"
/// class="anchor"></a></h2>`, so both halves the table of contents needs are
/// already in the markup and nothing has to re-parse the markdown.
#[cfg(feature = "ssr")]
fn headings(body_html: &str) -> Vec<Heading> {
    let mut found = Vec::new();
    let mut rest = body_html;

    while let Some(start) = rest.find("<h2 id=\"") {
        rest = &rest[start + 8..];
        let Some(end) = rest.find('"') else { break };
        let id = rest[..end].to_owned();

        let Some(text) = rest
            .find("data-heading-content=\"")
            .map(|at| &rest[at + 22..])
            .and_then(|after| after.find('"').map(|end| after[..end].to_owned()))
        else {
            break;
        };

        found.push(Heading {
            id,
            text: unescape(&text),
        });
    }

    found
}

/// Undo the entity escaping comrak applies to an attribute value.
///
/// `data-heading-content` is written through comrak's `escape`, and leptos
/// escapes again when the value renders as a text node. Without this a heading
/// titled `Cats & Dogs` reaches the rail as `Cats &amp; Dogs`. Comrak escapes
/// exactly these four, so there is nothing else to reverse.
#[cfg(feature = "ssr")]
fn unescape(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
}

/// The header image a post page should show, if any.
///
/// `None` for a post that embeds one of the sketches: the page already opens
/// with the live canvas, and the frontmatter image is a still of it.
#[cfg(feature = "ssr")]
fn hero(post: &'static crate::content::Post) -> Option<crate::content::Image> {
    post.image
        .filter(|_| !post.body_html.contains("src=\"/sketch/"))
}

/// Look up a post on the server.
///
/// The compiled `POSTS` table exists only in the `ssr` build, which keeps
/// ~124KB of rendered HTML out of the WASM bundle. A server function reaches it
/// on the client's behalf, so the client renders posts anyway. On first load
/// the resolved value is serialized into the page, so hydration does not make a
/// second request; only a client-side navigation to a different post does.
#[server]
pub async fn fetch_post(slug: String) -> Result<Option<PostView>, ServerFnError> {
    use crate::content::{self, Kind};

    let Some(post) = content::find(&slug) else {
        return Ok(None);
    };

    let siblings = post
        .series
        .map(|name| {
            content::series(name)
                .into_iter()
                .map(|entry| SeriesEntry {
                    slug: entry.slug.to_owned(),
                    title: entry.title.to_owned(),
                    part: entry.part,
                })
                .collect()
        })
        .unwrap_or_default();

    // `published` is already newest-first, so the entry before this one in the
    // list is the newer post and the entry after it is the older one.
    let ordered: Vec<_> = content::published(post.kind).collect();
    let at = ordered.iter().position(|other| other.slug == post.slug);
    let neighbour = |index: Option<usize>| {
        index
            .and_then(|index| ordered.get(index))
            .map(|other| Neighbour {
                slug: other.slug.to_owned(),
                title: other.title.to_owned(),
            })
    };

    Ok(Some(PostView {
        slug: post.slug.to_owned(),
        title: post.title.to_owned(),
        body_html: post.body_html.to_owned(),
        reading_minutes: post.reading_minutes,
        published: post.published.map(ToOwned::to_owned),
        category: post.category.map(ToOwned::to_owned),
        technologies: post.technologies.iter().map(|&it| it.to_owned()).collect(),
        show_reading_time: post.kind == Kind::Blog,
        // A post that embeds its own sketch opens with a live canvas. The
        // header image is a still of that same canvas, so showing both puts two
        // pictures of one thing above the fold. The listings keep it: there the
        // card has no canvas to lose out to.
        image_src: crate::pages::post::hero(post).map(|image| image.src.to_owned()),
        image_alt: crate::pages::post::hero(post).map(|image| image.alt.to_owned()),
        series: post.series.map(ToOwned::to_owned),
        headings: headings(post.body_html),
        siblings,
        next: neighbour(at.and_then(|at| at.checked_sub(1))),
        previous: neighbour(at.map(|at| at + 1)),
    }))
}

/// The date portion of an RFC 3339 timestamp.
fn published_date(published: Option<&str>) -> Option<&str> {
    published.map(|value| value.split('T').next().unwrap_or(value))
}

/// Post entry.
#[component]
pub fn Post() -> impl IntoView {
    let params = use_params::<PostParams>();
    let slug =
        move || params.with(|params| params.as_ref().ok().and_then(|params| params.post.clone()));

    let post = Resource::new(slug, |slug| async move {
        match slug {
            Some(slug) => fetch_post(slug).await,
            None => Ok(None),
        }
    });

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(Some(post)) => EitherOf3::A(view! { <PostBody post /> }),
                    Ok(None) => EitherOf3::B(view! { <NotFound /> }),
                    Err(_) => {
                        EitherOf3::C(
                            // Kept apart from the `None` arm: the post may well exist and
                            // the fetch just failed, so telling the reader the URL is
                            // wrong sends them away from a page that would load on a
                            // retry.
                            view! {
                                <Title text="Couldn't load this post" />
                                <div class="py-16 text-center">
                                    <h1 class="mb-3 font-mono text-3xl font-bold tracking-tight">
                                        "Couldn't load this post."
                                    </h1>
                                    <p class="text-ink-dim">
                                        "Something went wrong on our end. Try reloading the page."
                                    </p>
                                </div>
                            },
                        )
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
fn PostBody(post: PostView) -> impl IntoView {
    view! {
        <Title text=post.title.clone() />

        // The rail is unconditional. Reserving the column only when there is a
        // table of contents or a series would shift the article left and right
        // as the reader moves between posts, so the column stays even on a post
        // that leaves it empty.
        <div class="grid gap-12 lg:gap-x-8 lg:grid-cols-[236px_minmax(0,1fr)]">
            <aside class="hidden lg:block">
                <div class="flex sticky top-8 flex-col gap-7 text-[13px]">
                    <A
                        href=ROUTES.blog
                        attr:class="inline-flex gap-1.5 items-center font-mono text-xs"
                    >
                        <BackArrowIcon class="size-3" />
                        "all posts"
                    </A>

                    <PostRailToc headings=post.headings.clone() />
                    <PostRailSeries
                        series=post.series.clone()
                        siblings=post.siblings.clone()
                        current=post.slug.clone()
                    />
                </div>
            </aside>

            // The rule hangs off the article rather than the rail so it runs
            // the full height of the body. On the rail it would stop with the
            // sticky block, which is short and leaves the gap looking accidental.
            <article class="min-w-0 lg:pl-8 lg:border-l lg:border-rule">
                // Mono at 44px fit about 28 characters across the article
                // column, and titles here run to 77, so the longest broke over
                // three ragged lines. 32px fits ~38 and `text-balance` evens
                // out what still wraps.
                <h1 class="mb-4 font-mono text-2xl font-bold tracking-tighter leading-tight text-balance sm:text-[2rem]">
                    {post.title.clone()}
                </h1>

                // Field by field rather than the whole `PostView`: that struct
                // owns `body_html`, and the largest post is ~100KB to clone for
                // a one-line strip.
                <PostMeta
                    category=post.category.clone()
                    technologies=post.technologies.clone()
                    published=post.published.clone()
                    reading_minutes=post.reading_minutes
                    show_reading_time=post.show_reading_time
                />

                {post
                    .image_src
                    .clone()
                    .map(|src| {
                        view! {
                            <img
                                src=src
                                alt=post.image_alt.clone().unwrap_or_default()
                                class="object-cover mb-10 w-full max-h-80 rounded border border-rule"
                            />
                        }
                    })}

                // Pre-rendered at build time from markdown we control.
                <div class="prose" inner_html=post.body_html.clone()></div>

                <PostNeighbours previous=post.previous.clone() next=post.next.clone() />
            </article>
        </div>
    }
}

/// Taxonomy, date and reading time, in one line beneath the title.
///
/// The first slot says what kind of thing this is: a category on a blog post,
/// which links to the filtered listing, and the technologies on a project, which
/// are display text with nothing behind them. No post carries both.
#[component]
fn PostMeta(
    category: Option<String>,
    technologies: Vec<String>,
    published: Option<String>,
    reading_minutes: usize,
    show_reading_time: bool,
) -> impl IntoView {
    let date = published_date(published.as_deref()).map(ToOwned::to_owned);
    let technologies = technologies.join(" · ");

    view! {
        <div class="flex flex-wrap gap-4 items-center mb-8 font-mono text-xs text-ink-dim">
            {category
                .map(|category| {
                    let href = category_href(ROUTES.blog, &category);
                    // Only blog posts carry a category, so the filter always
                    // lands on the blog listing.
                    view! {
                        <A
                            href=href
                            attr:class="tracking-widest uppercase no-underline text-primary hover:text-accent"
                        >
                            {category}
                        </A>
                    }
                })}
            {(!technologies.is_empty())
                .then(|| {
                    view! {
                        <span class="tracking-widest uppercase text-primary">{technologies}</span>
                    }
                })} {date.map(|date| view! { <span>{date}</span> })}
            {show_reading_time.then(|| view! { <span>{reading_minutes} " min read"</span> })}
        </div>
    }
}

#[component]
fn PostRailToc(headings: Vec<Heading>) -> impl IntoView {
    if headings.is_empty() {
        return None;
    }

    Some(view! {
        <nav>
            <p class="mb-3 font-mono tracking-widest uppercase text-ink-dim text-[11px]">
                "Contents"
            </p>
            <ul class="flex flex-col border-l border-rule">
                {headings
                    .into_iter()
                    .map(|heading| {
                        view! {
                            <li>
                                <a
                                    href=format!("#{}", heading.id)
                                    class="block py-1.5 pl-3.5 -ml-px no-underline border-l-2 border-transparent hover:no-underline text-ink-dim hover:border-accent"
                                >
                                    {heading.text}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    })
}

#[component]
fn PostRailSeries(
    series: Option<String>,
    siblings: Vec<SeriesEntry>,
    current: String,
) -> impl IntoView {
    // A series of one is the post itself, which is not worth a nav.
    if siblings.len() < 2 {
        return None;
    }

    let name = series.unwrap_or_default();

    Some(view! {
        <nav class="pt-6 border-t border-rule">
            <p class="mb-3 font-mono tracking-widest uppercase text-ink-dim text-[11px]">
                {name} " series"
            </p>
            <ul class="flex flex-col gap-2">
                {siblings
                    .into_iter()
                    .map(|entry| {
                        let label = entry
                            .part
                            .map_or_else(|| "Intro".to_owned(), |part| part.to_string());
                        let is_current = entry.slug == current;
                        view! {
                            <li class="flex gap-2.5 items-baseline">
                                <span class="w-10 font-mono text-[11px] text-primary shrink-0">
                                    {label}
                                </span>
                                {if is_current {
                                    Either::Left(
                                        view! {
                                            <span
                                                class="font-medium line-clamp-2 text-ink"
                                                aria-current="page"
                                            >
                                                {entry.title}
                                            </span>
                                        },
                                    )
                                } else {
                                    Either::Right(
                                        view! {
                                            <A
                                                href=format!("/{}", entry.slug)
                                                attr:class="line-clamp-2 text-ink-dim hover:text-accent"
                                            >
                                                {entry.title}
                                            </A>
                                        },
                                    )
                                }}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    })
}

#[component]
fn PostNeighbours(previous: Option<Neighbour>, next: Option<Neighbour>) -> impl IntoView {
    if previous.is_none() && next.is_none() {
        return None;
    }

    // The oldest post has no previous, so its lone Next card would otherwise
    // sit in the left column while its own text is right-aligned. Starting it
    // in the second column keeps forward navigation on the right either way.
    let next_column = if previous.is_none() {
        "sm:col-start-2"
    } else {
        ""
    };

    Some(view! {
        <nav class="grid gap-4 mt-16 sm:grid-cols-2">
            {previous
                .map(|entry| {
                    view! {
                        <A
                            href=format!("/{}", entry.slug)
                            attr:class="block px-5 py-4 rounded border border-rule bg-panel text-ink no-underline hover:border-accent hover:no-underline"
                        >
                            <span class="flex gap-1.5 items-center mb-2 font-mono tracking-widest uppercase text-ink-dim text-[11px]">
                                <BackArrowIcon class="size-3" />
                                "Previous"
                            </span>
                            <span class="block font-semibold leading-snug">{entry.title}</span>
                        </A>
                    }
                })}
            {next
                .map(|entry| {
                    view! {
                        <A
                            href=format!("/{}", entry.slug)
                            attr:class=format!(
                                "block px-5 py-4 rounded border sm:text-right border-rule bg-panel text-ink no-underline hover:border-accent hover:no-underline {next_column}",
                            )
                        >
                            <span class="flex gap-1.5 items-center mb-2 font-mono tracking-widest uppercase sm:justify-end text-ink-dim text-[11px]">
                                "Next" <ArrowIcon class="size-3" />
                            </span>
                            <span class="block font-semibold leading-snug">{entry.title}</span>
                        </A>
                    }
                })}
        </nav>
    })
}
