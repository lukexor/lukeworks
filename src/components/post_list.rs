//! Shared post listing for `/blog` and `/projects`.
//!
//! Both pages are the same view over a different `kind`, so the data types, the
//! server function and the markup all live here rather than being written twice.

use crate::lukeworks::ROUTES;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_query_map};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::{Deserialize, Serialize};

/// Everything outside the RFC 3986 unreserved set, the only characters a query
/// value may carry unescaped.
///
/// Space encodes to `%20` rather than `+`, because leptos_router decodes with
/// `percent_decode_str` and that does not treat `+` as a space.
const QUERY_VALUE: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// Which listing to fetch.
///
/// Mirrors `crate::content::Kind`, which cannot be used directly: it is emitted
/// by `build.rs` without `Serialize`, and a server function argument has to
/// cross the network boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostKind {
    Blog,
    Project,
}

impl PostKind {
    /// The route this kind is listed at.
    ///
    /// The rail links back to its own listing. Hardcoding `/blog` there would
    /// throw a reader off `/projects` the moment a project grows a category.
    const fn route(self) -> &'static str {
        match self {
            Self::Blog => ROUTES.blog,
            Self::Project => ROUTES.projects,
        }
    }
}

/// A link to `base` narrowed to one category.
///
/// The category comes from hand-written frontmatter, so it is encoded rather
/// than interpolated: a space or an `&` would otherwise split the query and the
/// filter would silently miss.
pub fn category_href(base: &str, category: &str) -> String {
    format!(
        "{base}?category={}",
        utf8_percent_encode(category, QUERY_VALUE)
    )
}

/// One row of a listing.
///
/// Deliberately excludes `body_html`. A listing of 26 posts would otherwise
/// serialize every rendered post into the page, which is the bulk of the site's
/// HTML — see the note on `fetch_post` about that tradeoff for a single post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSummary {
    /// Filename without extension, which is also the URL.
    pub slug: String,
    /// Post title.
    pub title: String,
    /// One-line blurb, from the frontmatter or derived from the body.
    pub description: String,
    /// Lowercase category from the frontmatter.
    pub category: Option<String>,
    /// What a project was built with. Empty on a blog post.
    pub technologies: Vec<String>,
    /// RFC 3339 publication timestamp.
    pub published: Option<String>,
    /// Derived at build time from the word count.
    pub reading_minutes: usize,
    /// Display name of the series this post belongs to.
    pub series: Option<String>,
}

/// A listing plus the counts its rail needs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Listing {
    /// Every published post of one kind, newest first.
    pub posts: Vec<PostSummary>,
    /// Post count per year, newest year first.
    pub years: Vec<(String, usize)>,
    /// Post count per category, most posts first.
    pub categories: Vec<(String, usize)>,
    /// The category [`Listing::posts`] was narrowed to, if any.
    pub selected: Option<String>,
}

/// Fetch a listing from the compiled post table.
///
/// Goes through a server function for the same reason `fetch_post` does: the
/// `POSTS` table is `ssr`-only, so a component reading it directly would render
/// on the server and then blank out during hydration. leptos serializes the
/// resolved value into the page, so the first load costs no extra request.
///
/// The counts are tallied here rather than in the view so the browser never
/// walks the whole list to render a sidebar.
///
/// `category` narrows the returned posts. Category counts stay whole, so the
/// rail keeps offering every category rather than collapsing to the one already
/// chosen. Year counts are tallied over the narrowed set instead, because each
/// one links to a `<h2>` anchor that only exists for a year still on the page.
#[server]
pub async fn list_posts(
    kind: PostKind,
    category: Option<String>,
) -> Result<Listing, ServerFnError> {
    use crate::content::Kind;
    use std::collections::BTreeMap;

    let kind = match kind {
        PostKind::Blog => Kind::Blog,
        PostKind::Project => Kind::Project,
    };

    // `published` already yields newest-first and filters drafts; build.rs did
    // the sorting, so nothing here parses a date to order the list.
    let all: Vec<_> = crate::content::published(kind)
        .map(|post| PostSummary {
            slug: post.slug.to_owned(),
            title: post.title.to_owned(),
            description: post.description.to_owned(),
            category: post.category.map(ToOwned::to_owned),
            technologies: post.technologies.iter().map(|&it| it.to_owned()).collect(),
            published: post.published.map(ToOwned::to_owned),
            reading_minutes: post.reading_minutes,
            series: post.series.map(ToOwned::to_owned),
        })
        .collect();

    let mut categories: BTreeMap<String, usize> = BTreeMap::new();
    for post in &all {
        if let Some(category) = &post.category {
            *categories.entry(category.clone()).or_default() += 1;
        }
    }
    let mut categories: Vec<_> = categories.into_iter().collect();
    categories.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let selected = category.filter(|name| !name.is_empty());
    let posts: Vec<_> = match &selected {
        Some(name) => all
            .into_iter()
            .filter(|post| post.category.as_deref() == Some(name.as_str()))
            .collect(),
        None => all,
    };

    let mut years: BTreeMap<String, usize> = BTreeMap::new();
    for post in &posts {
        if let Some(year) = published_year(post.published.as_deref()) {
            *years.entry(year.to_owned()).or_default() += 1;
        }
    }
    let mut years: Vec<_> = years.into_iter().collect();
    years.reverse();

    Ok(Listing {
        posts,
        years,
        categories,
        selected,
    })
}

/// The date portion of an RFC 3339 timestamp.
///
/// Frontmatter dates are whole days whose time component means nothing, so the
/// string is truncated rather than parsed. No timezone handling, and nothing to
/// go wrong at runtime.
fn published_date(published: Option<&str>) -> Option<&str> {
    published.map(|value| value.split('T').next().unwrap_or(value))
}

/// The year portion of an RFC 3339 timestamp.
fn published_year(published: Option<&str>) -> Option<&str> {
    published_date(published).and_then(|date| date.split('-').next())
}

/// The month and day, for a row that already sits under a year heading.
fn published_day(published: Option<&str>) -> Option<String> {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let date = published_date(published)?;
    let mut parts = date.split('-').skip(1);
    let month: usize = parts.next()?.parse().ok()?;
    let day: usize = parts.next()?.parse().ok()?;
    Some(format!("{} {day}", MONTHS.get(month.checked_sub(1)?)?))
}

/// Render the listing for one kind of post.
///
/// A `category` query parameter narrows the rows. The resource keys on it, so a
/// client-side navigation between filters refetches instead of showing the
/// previous set.
#[component]
pub fn PostList(kind: PostKind) -> impl IntoView {
    let query = use_query_map();
    let category = move || query.read().get("category").filter(|it| !it.is_empty());

    let listing = Resource::new(
        move || (kind, category()),
        |(kind, category)| list_posts(kind, category),
    );

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let listing = listing.await.unwrap_or_default();
                view! { <ListingBody listing kind /> }
            })}
        </Suspense>
    }
}

#[component]
fn ListingBody(listing: Listing, kind: PostKind) -> impl IntoView {
    // Group as we walk, which keeps the newest-first order build.rs already
    // established rather than re-sorting by a parsed date.
    let mut groups: Vec<(String, Vec<PostSummary>)> = Vec::new();
    for post in listing.posts {
        let year = published_year(post.published.as_deref())
            .unwrap_or("Undated")
            .to_owned();
        match groups.last_mut() {
            Some((current, rows)) if *current == year => rows.push(post),
            _ => groups.push((year, vec![post])),
        }
    }

    let empty = groups.is_empty();
    let selected = listing.selected.clone();

    view! {
        <div class="grid gap-12 lg:grid-cols-[236px_minmax(0,1fr)]">
            <ListingRail
                years=listing.years
                categories=listing.categories
                selected=listing.selected
                base=kind.route()
            />

            <div class="min-w-0">
                <Show when=move || empty>
                    <p class="text-ink-dim">
                        {selected
                            .clone()
                            .map_or_else(
                                || "Nothing published here yet.".to_owned(),
                                |name| format!("Nothing published under \"{name}\" yet."),
                            )}
                    </p>
                </Show>
                {groups
                    .into_iter()
                    .map(|(year, rows)| {
                        let anchor = format!("y{year}");
                        view! {
                            <section class="mb-10">
                                <div class="flex gap-4 items-center mb-1">
                                    <h2
                                        id=anchor
                                        class="font-mono text-3xl font-bold tracking-tighter text-numeral"
                                    >
                                        {year}
                                    </h2>
                                    <span class="flex-grow h-px bg-rule"></span>
                                </div>
                                <ul>
                                    {rows
                                        .into_iter()
                                        .map(|post| view! { <ListingRow post kind /> })
                                        .collect_view()}
                                </ul>
                            </section>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// One row of a listing.
///
/// `kind` decides whether the reading time shows. Every project sits at one or
/// two minutes, so the column says nothing there. A blog post ranges from four
/// to eighteen, which is worth knowing before clicking.
#[component]
fn ListingRow(post: PostSummary, kind: PostKind) -> impl IntoView {
    let eyebrow = post
        .category
        .clone()
        .unwrap_or_else(|| post.technologies.join(" · "));

    view! {
        <li class="border-t border-rule">
            <A
                href=format!("/{}", post.slug)
                attr:class="grid gap-x-6 gap-y-1.5 py-5 no-underline text-ink sm:grid-cols-[76px_minmax(0,1fr)_auto] hover:no-underline group"
            >
                <span class="font-mono text-xs sm:mt-1.5 text-ink-dim">
                    {published_day(post.published.as_deref())}
                </span>
                <span class="min-w-0">
                    // Amber on a touch screen. There is no hover state to lean
                    // on there, and a title in body colour reads as prose rather
                    // than as the link the whole row is.
                    <span class="block text-lg font-medium tracking-tight pointer-coarse:text-accent group-hover:text-accent">
                        {post.title}
                    </span>
                    <span class="block mt-1 max-w-prose text-sm leading-relaxed text-ink-dim">
                        {post.description}
                    </span>
                    <span class="flex flex-wrap gap-3 items-center mt-2 font-mono text-[11px]">
                        {(!eyebrow.is_empty())
                            .then(|| {
                                view! {
                                    <span class="tracking-widest uppercase text-primary">
                                        {eyebrow}
                                    </span>
                                }
                            })}
                        {post
                            .series
                            .map(|series| {
                                view! {
                                    <span class="py-0.5 px-1.5 rounded-sm border text-ink-dim border-rule">
                                        {series}
                                    </span>
                                }
                            })}
                    </span>
                </span>
                {(kind == PostKind::Blog)
                    .then(|| {
                        view! {
                            <span class="font-mono text-xs sm:mt-1.5 sm:text-right text-ink-dim">
                                {post.reading_minutes} " min"
                            </span>
                        }
                    })}
            </A>
        </li>
    }
}

/// Archive and category navigation for one listing.
///
/// `base` is the listing's own route, so `/projects` narrows to
/// `/projects?category=…` rather than sending the reader to `/blog`.
///
/// Each nav is suppressed when it has nothing to show. On the error path
/// `unwrap_or_default` yields empty counts, and a bare "Archive" heading with no
/// rows under it reads as broken rather than empty. The grid column stays either
/// way, so the listing does not shift.
#[component]
fn ListingRail(
    years: Vec<(String, usize)>,
    categories: Vec<(String, usize)>,
    selected: Option<String>,
    base: &'static str,
) -> impl IntoView {
    let has_years = !years.is_empty();
    let has_categories = !categories.is_empty();
    let filtering = selected.is_some();
    view! {
        <aside class="hidden lg:block">
            <div class="flex sticky top-8 flex-col gap-8 text-[13px]">
                <Show when=move || has_years>
                    <nav>
                        <p class="mb-3 font-mono tracking-widest uppercase text-ink-dim text-[11px]">
                            "Archive"
                        </p>
                        <ul class="flex flex-col">
                            {years
                                .clone()
                                .into_iter()
                                .map(|(year, count)| {
                                    let anchor = format!("#y{year}");
                                    view! {
                                        <li>
                                            <a
                                                href=anchor
                                                class="flex justify-between py-1.5 px-2.5 font-mono no-underline rounded-sm hover:no-underline text-ink-dim hover:bg-panel hover:text-accent"
                                            >
                                                <span>{year}</span>
                                                <span>{count}</span>
                                            </a>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </nav>
                </Show>

                <Show when=move || has_categories>
                    <nav>
                        <p class="mb-3 font-mono tracking-widest uppercase text-ink-dim text-[11px]">
                            "Category"
                        </p>
                        <ul class="flex flex-col">
                            {categories
                                .clone()
                                .into_iter()
                                .map(|(category, count)| {
                                    let active = selected.as_deref() == Some(category.as_str());
                                    let href = if active {
                                        base.to_owned()
                                    } else {
                                        category_href(base, &category)
                                    };
                                    let class = if active {
                                        "flex justify-between py-1.5 px-2.5 font-mono no-underline rounded-sm hover:no-underline bg-panel text-accent"
                                    } else {
                                        "flex justify-between py-1.5 px-2.5 font-mono no-underline rounded-sm hover:no-underline text-ink-dim hover:bg-panel hover:text-accent"
                                    };
                                    // Clicking the category already showing clears
                                    // the filter, so the row doubles as its own
                                    // off switch.
                                    view! {
                                        <li>
                                            <A
                                                href=href
                                                attr:class=class
                                                attr:aria-current=active.then_some("true")
                                            >
                                                <span>{category}</span>
                                                <span>{count}</span>
                                            </A>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                        <Show when=move || filtering>
                            <A
                                href=base
                                attr:class="inline-block mt-2 px-2.5 font-mono text-[11px] tracking-widest uppercase"
                            >
                                "Clear filter"
                            </A>
                        </Show>
                    </nav>
                </Show>
            </div>
        </aside>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_category_href_encodes_what_would_split_the_query() {
        assert_eq!(
            category_href(ROUTES.blog, "video games"),
            "/blog?category=video%20games"
        );
        assert_eq!(category_href(ROUTES.blog, "r&d"), "/blog?category=r%26d");
        // The unreserved characters stay legible.
        assert_eq!(
            category_href(ROUTES.projects, "video-games"),
            "/projects?category=video-games"
        );
    }

    #[test]
    fn each_kind_links_back_to_its_own_listing() {
        assert_eq!(PostKind::Blog.route(), ROUTES.blog);
        assert_eq!(PostKind::Project.route(), ROUTES.projects);
    }

    #[test]
    fn published_date_drops_the_time_component() {
        assert_eq!(
            published_date(Some("2020-01-31T21:19:14Z")),
            Some("2020-01-31")
        );
        // A frontmatter date may be written without a time component.
        assert_eq!(published_date(Some("2020-01-31")), Some("2020-01-31"));
        assert_eq!(published_date(None), None);
    }

    #[test]
    fn published_year_and_day_split_a_timestamp() {
        assert_eq!(published_year(Some("2020-01-31T21:19:14Z")), Some("2020"));
        assert_eq!(
            published_day(Some("2020-01-31T21:19:14Z")),
            Some("Jan 31".to_owned())
        );
        // A leading zero in the day reads as the bare number.
        assert_eq!(
            published_day(Some("2014-11-05T00:00:00Z")),
            Some("Nov 5".to_owned())
        );
        assert_eq!(published_day(None), None);
    }

    #[test]
    fn a_malformed_date_yields_nothing_rather_than_panicking() {
        // Frontmatter is hand-written, so a short or non-numeric date reaches
        // this without passing through a date parser first.
        assert_eq!(published_day(Some("2020")), None);
        assert_eq!(published_day(Some("2020-ab-cd")), None);
        assert_eq!(published_day(Some("2020-13-01")), None);
    }
}
