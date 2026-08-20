//! Landing page.

use crate::{
    components::icons::{ArrowIcon, DownArrowIcon},
    lukeworks::{ROUTES, SUPPORT_EMAIL},
};
use leptos::{either::Either, prelude::*};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

/// One recent post, as the homepage row shows it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentPost {
    slug: String,
    title: String,
    description: String,
    category: Option<String>,
    published: Option<String>,
    reading_minutes: usize,
}

/// One featured project, as the homepage card shows it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturedProject {
    slug: String,
    title: String,
    description: String,
    technologies: Vec<String>,
    image_src: Option<String>,
    image_alt: Option<String>,
}

/// Everything below the hero.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HomeView {
    recent: Vec<RecentPost>,
    featured: Vec<FeaturedProject>,
}

/// Fetch both homepage sections in one call.
///
/// One server function rather than two calls into `list_posts`, because the
/// homepage wants neither the archive counts nor the category counts a
/// `Listing` carries, and those get serialized into the page whether or not
/// anything renders them.
#[server]
pub async fn fetch_home() -> Result<HomeView, ServerFnError> {
    use crate::content::{self, Kind};

    /// Recent posts the homepage lists.
    const RECENT_POSTS: usize = 4;
    /// Projects the homepage cards.
    const FEATURED_PROJECTS: usize = 3;

    let recent = content::published(Kind::Blog)
        .take(RECENT_POSTS)
        .map(|post| RecentPost {
            slug: post.slug.to_owned(),
            title: post.title.to_owned(),
            description: post.description.to_owned(),
            category: post.category.map(ToOwned::to_owned),
            published: post.published.map(ToOwned::to_owned),
            reading_minutes: post.reading_minutes,
        })
        .collect();

    // Falling back to the newest projects keeps the section from emptying out
    // if the flag is ever dropped from the frontmatter.
    let mut projects: Vec<_> = content::published(Kind::Project)
        .filter(|post| post.featured)
        .collect();
    if projects.is_empty() {
        projects = content::published(Kind::Project).collect();
    }

    let featured = projects
        .into_iter()
        .take(FEATURED_PROJECTS)
        .map(|post| FeaturedProject {
            slug: post.slug.to_owned(),
            title: post.title.to_owned(),
            description: post.description.to_owned(),
            technologies: post.technologies.iter().map(|&it| it.to_owned()).collect(),
            image_src: post.image.map(|image| image.src.to_owned()),
            image_alt: post.image.map(|image| image.alt.to_owned()),
        })
        .collect();

    Ok(HomeView { recent, featured })
}

/// The date of an RFC 3339 timestamp, dotted.
///
/// `2020-01-31T21:19:14Z` reads as `2020.01.31`. Dots rather than dashes so the
/// date sits as one glyph run against the mono titles beside it.
fn dotted_date(published: Option<&str>) -> Option<String> {
    let date = published?.split('T').next()?;
    Some(date.replace('-', "."))
}

/// Landing page.
#[component]
pub fn Home() -> impl IntoView {
    let home = Resource::new(|| (), |()| fetch_home());

    view! {
        // Every route sets one. Without it the tab shows the bare URL.
        <Title text="Home" />

        <Hero />

        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                let home = home.await.unwrap_or_default();
                view! {
                    <RecentWriting posts=home.recent />
                    <FeaturedWork projects=home.featured />
                }
            })}
        </Suspense>

        <HomeContact />
    }
}

/// Full-width introduction over a blurred code backdrop.
///
/// Rendered outside the `Suspense` because it reads no post data, so it paints
/// with the first byte instead of waiting on the resource.
#[component]
fn Hero() -> impl IntoView {
    view! {
        // `<main>` centres the page and pads it. The hero has to reach the
        // container edges to read as a band, so it cancels that padding and puts
        // its own back on the inside.
        <section class="overflow-hidden relative -mx-6 mb-16 -mt-10 border-b sm:-mx-14 border-rule">
            // The site's own source, escaped at build time by `build.rs`.
            // Decorative: a 2px blur, which softens the glyphs without turning
            // them to smear, and the scrim below does the dimming. Text rather
            // than an image, so it arrives with the document, scales at any
            // width, and takes its one colour from `.code-backdrop`.
            <div
                aria-hidden="true"
                class="absolute inset-0 blur-[2px] code-backdrop"
                inner_html=crate::backdrop::CODE_BACKDROP
            ></div>
            <div class="absolute inset-0 hero-scrim"></div>

            <div class="relative py-20 px-6 sm:py-24 sm:px-14">
                <p class="mb-5 font-mono text-[13px] text-primary">"$ whoami"</p>
                <h1 class="mb-5 font-mono text-4xl font-bold tracking-tighter leading-none sm:text-6xl text-balance">
                    "Hi, I'm Luke" <span class="text-accent">"_"</span>
                </h1>
                <p class="mb-10 font-mono text-lg tracking-tight sm:text-xl text-ink">
                    "Software Engineer. Designer. Thinker."
                </p>
                <div class="flex flex-wrap gap-4 items-center">
                    // Scrolls to the recent posts rather than leaving for
                    // `/blog`, which the section's own "all posts" link already
                    // does. The router intercepts this like any same-origin
                    // anchor and re-implements the fragment scroll in
                    // `scroll_to_el`, so it needs no `rel="external"`.
                    <a
                        href="#recent"
                        class="inline-flex gap-2.5 items-center py-3 px-6 font-mono text-sm font-bold no-underline rounded-sm hover:no-underline bg-accent text-on-accent hover:bg-accent-hover"
                    >
                        "Explore"
                        <DownArrowIcon />
                    </a>
                    <span class="font-mono text-[13px] text-ink-dim">"Have a look around!"</span>
                </div>
            </div>
        </section>
    }
}

/// Section heading with a link to the full listing.
#[component]
fn SectionHead(label: &'static str, href: &'static str, more: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-baseline mb-7">
            <h2 class="font-mono text-xs font-medium uppercase tracking-[0.18em] text-ink-dim">
                {label}
            </h2>
            // An SVG arrow, because neither vendored font subset covers `→` and
            // a text one falls back to a system font mid-string.
            <A href=href attr:class="inline-flex gap-1.5 items-center font-mono text-[13px]">
                {more}
                <ArrowIcon class="size-3" />
            </A>
        </div>
    }
}

#[component]
fn RecentWriting(posts: Vec<RecentPost>) -> impl IntoView {
    if posts.is_empty() {
        return None;
    }

    Some(view! {
        // The hero's Explore button scrolls here.
        <section id="recent" class="mb-16 scroll-mt-8">
            <SectionHead label="Recent writing" href=ROUTES.blog more="all posts" />

            // The last row carries the closing rule, so the block reads as a
            // table rather than a list that stops mid-air.
            <div class="border-b border-rule">
                {posts.into_iter().map(|post| view! { <RecentRow post /> }).collect_view()}
            </div>
        </section>
    })
}

#[component]
fn RecentRow(post: RecentPost) -> impl IntoView {
    view! {
        <A
            href=format!("/{}", post.slug)
            attr:class="grid gap-x-6 gap-y-1.5 py-5 no-underline border-t text-ink border-rule sm:grid-cols-[104px_minmax(0,1fr)_128px_64px] hover:no-underline group"
        >
            <span class="font-mono text-xs sm:mt-1.5 text-ink-dim">
                {dotted_date(post.published.as_deref())}
            </span>
            <span class="min-w-0">
                // Amber on a touch screen, where there is no hover state to
                // carry the fact that the whole row is a link.
                <span class="block text-lg font-medium tracking-tight pointer-coarse:text-accent group-hover:text-accent">
                    {post.title}
                </span>
                <span class="block mt-1 max-w-prose text-sm leading-relaxed text-ink-dim">
                    {post.description}
                </span>
            </span>
            <span class="font-mono tracking-widest uppercase sm:mt-1.5 text-primary text-[11px]">
                {post.category}
            </span>
            <span class="font-mono text-xs sm:mt-1.5 sm:text-right text-ink-dim">
                {post.reading_minutes} " min"
            </span>
        </A>
    }
}

#[component]
fn FeaturedWork(projects: Vec<FeaturedProject>) -> impl IntoView {
    if projects.is_empty() {
        return None;
    }

    Some(view! {
        <section class="mb-16">
            <SectionHead label="Things I've built" href=ROUTES.projects more="all projects" />

            <ul class="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
                {projects
                    .into_iter()
                    .map(|project| view! { <ProjectCard project /> })
                    .collect_view()}
            </ul>
        </section>
    })
}

#[component]
fn ProjectCard(project: FeaturedProject) -> impl IntoView {
    let slug = project.slug.clone();

    view! {
        <li>
            <A
                href=format!("/{}", project.slug)
                attr:class="flex overflow-hidden flex-col h-full no-underline rounded border bg-panel border-rule text-ink hover:border-accent hover:no-underline group"
            >
                {match project.image_src {
                    None => {
                        Either::Left(
                            // Four posts carry no image. A tinted band with the slug on
                            // it keeps the card the same height as its neighbours,
                            // rather than leaving a card that starts halfway down.
                            view! {
                                <div class="flex justify-center items-center h-32 border-b border-rule">
                                    <span class="font-mono text-xs text-primary">
                                        "$ open " {slug}
                                    </span>
                                </div>
                            },
                        )
                    }
                    Some(src) => {
                        Either::Right(
                            view! {
                                // The cards sit well below the fold, so these
                                // load only once they are scrolled toward and
                                // stop competing with the hero for bandwidth.
                                <img
                                    src=src
                                    alt=project.image_alt.unwrap_or_default()
                                    loading="lazy"
                                    decoding="async"
                                    class="object-cover w-full h-32 group-hover:opacity-100 opacity-85"
                                />
                            },
                        )
                    }
                }}

                <div class="flex flex-col flex-grow gap-1.5 py-4 px-5">
                    {(!project.technologies.is_empty())
                        .then(|| {
                            view! {
                                <span class="font-mono tracking-widest uppercase text-primary text-[11px]">
                                    {project.technologies.join(" · ")}
                                </span>
                            }
                        })} // A card holds a third of the grid, and nothing bounds
                    // either field at the source. Clamping both keeps three
                    // cards the same shape whatever lands in them.
                    <span class="font-medium line-clamp-2 pointer-coarse:text-accent group-hover:text-accent">
                        {project.title}
                    </span>
                    <span class="leading-relaxed line-clamp-3 text-[13px] text-ink-dim">
                        {project.description}
                    </span>
                </div>
            </A>
        </li>
    }
}

/// Closing invitation to get in touch.
#[component]
fn HomeContact() -> impl IntoView {
    view! {
        <section class="pt-10 mb-4 border-t border-rule">
            <p class="mb-3 font-mono text-[13px] text-primary">"$ contact --help"</p>
            <p class="max-w-2xl leading-relaxed text-ink-dim">
                "Have a question or want to work together? Drop me a line at "
                <a href=format!("mailto:{SUPPORT_EMAIL}")>{SUPPORT_EMAIL}</a> ", or "
                <A href=ROUTES.about>"read more about me"</A> "."
            </p>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_timestamp_reads_as_a_dotted_date() {
        assert_eq!(
            dotted_date(Some("2020-01-31T21:19:14Z")).as_deref(),
            Some("2020.01.31")
        );
        // A frontmatter date may be written without a time component.
        assert_eq!(
            dotted_date(Some("2022-05-02")).as_deref(),
            Some("2022.05.02")
        );
        assert_eq!(dotted_date(None), None);
    }
}
