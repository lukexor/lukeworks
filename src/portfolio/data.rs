//! Portfolio static data.

use crate::{
    hashmap,
    portfolio::{
        contact::Contact,
        errors::Error,
        homepage::Intro,
        post::{Blog, Meta, Project},
        Layout,
    },
};
use leptos::{view, IntoView};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use uuid::uuid;

/// URL string.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Url(pub &'static str);

/// Date string.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct DateStr(pub &'static str);

/// Icon attributes.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct IconAttrs {
    pub icon: &'static str,
    pub title: &'static str,
    pub href: &'static str,
}

impl IconAttrs {
    /// Create a new `IconAttrs`.
    pub const fn new(icon: &'static str, title: &'static str, href: &'static str) -> Self {
        Self { icon, title, href }
    }
}

/// [Image] attributes.
///
/// [Image]: crate::components::Image
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct ImageAttrs {
    pub alt: &'static str,
    pub width: u32,
    pub height: u32,
    pub src: Url,
}

impl ImageAttrs {
    /// Create a new `ImageAttrs`.
    pub const fn new(alt: &'static str, width: u32, height: u32, src: Url) -> Self {
        Self {
            alt,
            width,
            height,
            src,
        }
    }
}

impl Default for ImageAttrs {
    fn default() -> Self {
        Self {
            src: Url("/images/placeholder.webp"),
            alt: "Placeholder image",
            // TODO: Placeholder image size
            width: 200,
            height: 200,
        }
    }
}

/// Site index.html metadata.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct SiteMeta {
    pub title: &'static str,
    pub description: &'static str,
    pub origin: &'static str,
}

/// Portfolio metadata.
pub const META: SiteMeta = SiteMeta {
    title: "Lucas Petherbridge | Software Engineer",
    description: concat!(
        "A blog and project portfolio by Lucas Petherbridge on programming, technology, ",
        "and video game topics."
    ),
    origin: "https://lukeworks.tech",
};

/// URL Route.
#[derive(Debug)]
#[must_use]
pub struct Route {
    pub title: &'static str,
    pub path: &'static str,
}

impl Route {
    pub const fn new(title: &'static str, path: &'static str) -> Self {
        Self { title, path }
    }
}

/// Portfolio URL Routes.
pub static ROUTES: Lazy<HashMap<&'static str, Route>> = Lazy::new(|| {
    hashmap! {
        "home" => Route::new("Home", "/"),
        "blog" => Route::new("Blog", "/#blog"),
        "projects" => Route::new("Projects", "/#projects"),
        "about" => Route::new("About", "/#about"),
        "contact" => Route::new("Contact", "/#contact"),
        "post" => Route::new("Post Entry", "/:postSlug"),
        "resume" => Route::new("Resume", "/resume"),
        "tetanes-web" => Route::new("TetaNES Web", "/tetanes-web"),
        "p5js" => Route::new("P5JS Project", "/p5js/:title"),
    }
});

/// Portfolio layout static copy.
pub const LAYOUT: Layout = Layout {
    logo: "L",
    search_placeholder: "Search...",
    social_icons: [
        IconAttrs::new("github", "GitHub", "http://github.com/lukexor"),
        IconAttrs::new(
            "linkedin",
            "LinkedIn",
            "https://linkedin.com/in/lucaspetherbridge",
        ),
        IconAttrs::new(
            "rss",
            "RSS Feed",
            "https://feeds.feedburner.com/LucasPetherbridge",
        ),
        IconAttrs::new(
            "email",
            "Email me@lukeworks.tech",
            "mailto:me@lukeworks.tech",
        ),
    ],
};

/// Portfolio homepage intro static copy.
pub const INTRO: Intro = Intro {
    title: "Hi, I'm Luke",
    subtitle: "Software Engineer. Designer. Thinker.",
    action: IconAttrs::new("circle-chevron-down", "Have a look around!", "/#projects"),
};

/// Blog posts.
// TODO: descriptions, Image sizes, tags, move images to markdown
pub static BLOG_POSTS: Lazy<Vec<Blog>> = Lazy::new(|| {
    vec![
        Blog {
            meta: Meta {
                id: uuid!("491d14d0-69c5-4a5a-b9b2-75337869893d"),
                slug: "tetanes-part-2",
                title: "NES Emulation in Rust: Designs and Frustrations",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Nintendo Entertainment System console",
                    200,
                    200,
                    Url("/images/blog/nes_console.webp"),
                ),
                category: "programming",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/nes_console.webp"),
            //     Nintendo Entertainment System console",
            //     200,
            //     200,
            // }),
            minutes_to_read: 43,
            published: Some(DateStr("2020-01-31 21:19:14.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("4dec2631-5a2e-4595-bd51-3753aa572a95"),
                slug: "tetanes-part-1",
                title: "Programming an NES Emulator from Scratch in Rust",
                description: "",
                thumbnail: ImageAttrs::new(
                    "TetaNES Logo",
                    200,
                    200,
                    Url("/images/blog/tetanes.webp"),
                ),
                category: "Programming",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/tetanes.webp"),
            //     TetaNES Logo",
            //     200,
            //     200,
            // }),
            minutes_to_read: 10,
            published: Some(DateStr("2019-09-19 06:10:44.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("1e4c03f7-13fb-4f1e-9468-871f420eb7dd"),
                slug: "software-malaise",
                title: "Software Malaise",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Empty derelict room with a single chair",
                    200,
                    200,
                    Url("/images/blog/software_malaise.webp"),
                ),
                category: "Programming",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/software_malaise.webp"),
            //     Empty derelict room with a single chair",
            //     200,
            //     200,
            // }),
            minutes_to_read: 10,
            published: Some(DateStr("2017-03-27 05:36:10.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("47414a25-bd1f-4442-a5b2-b2d17c12cd08"),
                slug: "lost-and-found-part-5",
                title: r#""Lost and Found" : Part 5 - Turbo-charge your career and avoid stagnation"#,
                description: "",
                thumbnail: ImageAttrs::new(
                    "Lego Stormtrooper on a speeder",
                    200,
                    200,
                    Url("/images/blog/lost_and_found_part_5.webp"),
                ),
                category: "Career",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/lost_and_found_part_5.webp"),
            //     Lego Stormtrooper on a speeder",
            //     200,
            //     200,
            // }),
            minutes_to_read: 6,
            published: Some(DateStr("2015-03-18 18:56:01.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("05139fea-394a-44ea-943d-e8b365579ea3"),
                slug: "lost-and-found-part-4",
                title: r#""Lost and Found" : Part 4 - Software development in practice"#,
                description: "",
                thumbnail: ImageAttrs::new(
                    "Street signs: Success Ln., Failure Dr.",
                    200,
                    200,
                    Url("/images/blog/lost_and_found_part_4.webp"),
                ),
                category: "Career",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/lost_and_found_part_4.webp"),
            //     Street signs: Success Ln., Failure Dr.",
            //     200,
            //     200,
            // }),
            minutes_to_read: 14,
            published: Some(DateStr("2015-01-14 22:30:34.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("2aa5e8af-9f75-42d8-a21a-44c2a72dee47"),
                slug: "lost-and-found-part-3",
                title: r#""Lost and Found" : Part 3 - I do not think it means what you think it means"#,
                description: "",
                thumbnail: ImageAttrs::new(
                    "Inigo Montoya from Princess Bride",
                    200,
                    200,
                    Url("/images/blog/lost_and_found_part_3.webp"),
                ),
                category: "Career",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/lost_and_found_part_3.webp"),
            //     Inigo Montoya from Princess Bride",
            //     200,
            //     200,
            // }),
            minutes_to_read: 9,
            published: Some(DateStr("2014-12-18 21:49:53.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("cc149424-f142-4064-8f31-642a48604a54"),
                slug: "lost-and-found-part-2",
                title: r#""Lost and Found" : Part 2 - PC load letter?!"#,
                description: "",
                thumbnail: ImageAttrs::new(
                    "Broken printer in a field",
                    200,
                    200,
                    Url("/images/blog/lost_and_found_part_2.webp"),
                ),
                category: "Career",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/lost_and_found_part_2.webp"),
            //     Broken printer in a field",
            //     200,
            //     200,
            // }),
            minutes_to_read: 14,
            published: Some(DateStr("2014-12-04 03:06:26.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("f1d1209d-9229-4ec0-b5db-feb74500df1b"),
                slug: "lost-and-found-part-1",
                title: r#""Lost and Found" : Part 1 - From Dungeons & Dragons to HTML"#,
                description: "",
                thumbnail: ImageAttrs::new(
                    "multi-sided dice",
                    200,
                    200,
                    Url("/images/blog/lost_and_found_part_1.webp"),
                ),
                category: "Career",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/lost_and_found_part_1.webp"),
            //     multi-sided dice",
            //     200,
            //     200,
            // }),
            minutes_to_read: 6,
            published: Some(DateStr("2014-11-26 00:52:11.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("231bcc49-3f54-4020-b9b7-713817095a81"),
                slug: "lost-and-found-series",
                title: r#""Lost and Found" Series : Lessons Learned"#,
                description: "",
                thumbnail: ImageAttrs::new(
                    "We can draw lessons from the past, but we cannot live in it.",
                    200,
                    200,
                    Url("/images/blog/lost_and_found_part_0.webp"),
                ),
                category: "Career",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/lost_and_found_part_0.webp"),
            //     We can draw lessons from the past, but we cannot live in it.",
            //     200,
            //     200,
            // }),
            minutes_to_read: 2,
            published: Some(DateStr("2014-11-21 07:42:52.000000")),
        },
        Blog {
            meta: Meta {
                id: uuid!("2181259b-470b-4003-a393-97f831c1f806"),
                slug: "first-post",
                title: "First post",
                description: "",
                thumbnail: ImageAttrs::default(),
                category: "Programming",
                tags: HashSet::new(),
            },
            // image: None,
            minutes_to_read: 1,
            published: Some(DateStr("2014-11-18 00:00:00.000000")),
        },
    ]
});

/// Project posts.
pub static PROJECT_POSTS: Lazy<Vec<Project>> = Lazy::new(|| {
    vec![
        Project {
            meta: Meta {
                id: uuid!("03b5d942-a155-4e45-bb85-f00b255130af"),
                slug: "pix-engine",
                title: "PixEngine - A Cross-Platform Graphics Library",
                description: "",
                thumbnail: ImageAttrs::default(),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("https://github.com/lukexor/pix-engine")),
            started: DateStr("2019-09-29 23:14:15.000000"),
            completed: None,
        },
        Project {
            meta: Meta {
                id: uuid!("69f027b9-ca8b-4984-b798-f89f2d63004a"),
                slug: "tetanes",
                title: "TetaNES - An NES Emulator written in Rust",
                description: "",
                thumbnail: ImageAttrs::new(
                    "TetaNES Logo",
                    200,
                    200,
                    Url("/images/blog/tetanes.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/blog/tetanes.webp"),
            //     TetaNES Logo",
            //     200,
            //     200,
            // }),
            website: Some(Url("https://github.com/lukexor/tetanes")),
            started: DateStr("2019-04-01 00:00:00.000000"),
            completed: None,
        },
        Project {
            meta: Meta {
                id: uuid!("a2d94b72-1df6-4c4d-96a9-dc05705d8206"),
                slug: "fluid-simulation",
                title: "Fluid Simulation",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Fluid Simulation",
                    200,
                    200,
                    Url("/images/projects/fluid-simulation.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/fluid-simulation")),
            started: DateStr("2020-02-21 23:50:51.000000"),
            completed: Some(DateStr("2020-02-21 23:50:52.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("e9b612af-a2e8-4a70-9672-c5e6c47d559b"),
                slug: "fourier",
                title: "Discrete Fourier Transforms",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Discrete Fourier Transform",
                    200,
                    200,
                    Url("/images/projects/fourier.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/fourier")),
            started: DateStr("2020-02-12 22:44:58.000000"),
            completed: Some(DateStr("2020-02-12 22:44:59.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("24c7d94c-0174-40e9-baf3-cc89d9cee737"),
                slug: "asteroids",
                title: "Asteroids!",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Asteroids",
                    200,
                    200,
                    Url("/images/projects/asteroids.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/asteroids")),
            started: DateStr("2020-02-11 20:52:02.000000"),
            completed: Some(DateStr("2020-02-11 20:52:05.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("ef06e2ef-8a33-4748-b7f4-81b726ac6e30"),
                slug: "pong",
                title: "Pong",
                description: "",
                thumbnail: ImageAttrs::new("Pong", 200, 200, Url("/images/projects/pong.webp")),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/pong")),
            started: DateStr("2020-02-11 02:21:54.000000"),
            completed: Some(DateStr("2020-02-11 02:21:56.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("dd286805-b641-4c7c-94c1-9dd758e1edbc"),
                slug: "lorenz-attractor",
                title: "Lorenz Attractor",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Lorenz Attractor",
                    200,
                    200,
                    Url("/images/projects/lorenz-attractor.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/lorenz-attractor")),
            started: DateStr("2020-02-06 01:45:16.000000"),
            completed: Some(DateStr("2020-02-06 01:45:15.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("12beffea-b5a7-4655-8b3a-11f5f4bbea67"),
                slug: "raycasting-2d",
                title: "2D Raycasting",
                description: "",
                thumbnail: ImageAttrs::new(
                    "2D Raycasting",
                    200,
                    200,
                    Url("/images/projects/raycasting-2d.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/raycasting-2d")),
            started: DateStr("2020-02-06 00:22:09.000000"),
            completed: Some(DateStr("2020-02-06 00:22:13.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("5444e624-f889-46b7-bc03-539bf36b608b"),
                slug: "fireworks",
                title: "Fireworks!",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Fireworks",
                    200,
                    200,
                    Url("/images/projects/fireworks.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/fireworks")),
            started: DateStr("2020-02-03 12:00:00.000000"),
            completed: Some(DateStr("2020-02-05 22:40:57.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("90254ae4-59fe-45eb-9744-ca6779804356"),
                slug: "matrix",
                title: "The Matrix Has You",
                description: "",
                thumbnail: ImageAttrs::new(
                    "The Matrix",
                    200,
                    200,
                    Url("/images/projects/matrix.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/matrix")),
            started: DateStr("2020-01-30 19:59:02.000000"),
            completed: Some(DateStr("2020-01-30 19:59:03.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("cc9bdb88-47bb-4342-9c03-bcca9d4b21a5"),
                slug: "maze-astar",
                title: "Maze Generation and A* Search",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Maze A* Search",
                    200,
                    200,
                    Url("/images/projects/maze-astar.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("/sketch/maze-astar")),
            started: DateStr("2020-01-28 19:57:34.000000"),
            completed: Some(DateStr("2020-01-29 18:16:02.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("e41d3922-6333-4a4a-a5e5-f8fe9e4c4b94"),
                slug: "chatdot",
                title: "ChatDot",
                description: "",
                thumbnail: ImageAttrs::default(),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("https://github.com/lukexor/ChatDot")),
            started: DateStr("2017-04-24 02:45:29.000000"),
            completed: Some(DateStr("2017-06-19 02:45:43.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("a5109baf-2c7a-442f-9b83-75e3f181f68b"),
                slug: "bell-demodulator",
                title: "A Bell 103 Demodulator",
                description: "",
                thumbnail: ImageAttrs::default(),
                category: "",
                tags: HashSet::new(),
            },
            // image: None,
            website: Some(Url("https://github.com/lukexor/bell103_demodulator")),
            started: DateStr("2019-04-26 02:42:01.000000"),
            completed: Some(DateStr("2019-04-26 02:42:04.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("852ce423-8ac4-47e7-a41e-0cc9b7cd7941"),
                slug: "haskelltaire",
                title: "Haskelltaire",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Haskelltaire",
                    200,
                    200,
                    Url("/images/projects/haskelltaire.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/projects/haskelltaire.webp"),
            //     Haskelltaire",
            //     200,
            //     200,
            // }),
            website: Some(Url("https://github.com/lukexor/haskelltaire")),
            started: DateStr("2019-03-01 00:00:00.000000"),
            completed: Some(DateStr("2019-03-27 00:00:00.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("8ce4440f-3cee-4c57-8d1c-f0addc91fa34"),
                slug: "umflint-env-sci-club",
                title: "UM-Flint Environmental Science Club Website",
                description: "",
                thumbnail: ImageAttrs::new(
                    "UM-Flint Environmental Science Club Website",
                    200,
                    200,
                    Url("/images/projects/umflint_env_club.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/projects/umflint_env_club.webp"),
            //     UM-Flint Environmental Science Club Website",
            //     200,
            //     200,
            // }),
            website: Some(Url("https://umflint-env.lukeworks.tech/")),
            started: DateStr("2007-09-01 12:00:00.000000"),
            completed: Some(DateStr("2007-11-01 12:00:00.000000")),
        },
        Project {
            meta: Meta {
                id: uuid!("0c985e6c-1569-44d9-a286-0122758a8830"),
                slug: "personal-portfolio",
                title: "Personal Portfolio/Blog",
                description: "",
                thumbnail: ImageAttrs::new(
                    "Portfolio Website",
                    200,
                    200,
                    Url("/images/projects/portfolio_blog.webp"),
                ),
                category: "",
                tags: HashSet::new(),
            },
            // image: Some(ImageAttrs::new(
            //     Url("/images/projects/portfolio_blog.webp"),
            //     Portfolio Website",
            //     200,
            //     200,
            // }),
            website: Some(Url("https://lukeworks.tech/")),
            started: DateStr("2014-08-12 16:49:29.000000"),
            completed: Some(DateStr("2015-03-18 17:58:41.000000")),
        },
    ]
});

/// About static copy.
pub const ABOUT: &str = include_str!("../../data/about.md");

/// Contact static copy.
pub const CONTACT: Contact = Contact {
    name: "Lucas Petherbridge",
    email: "me@lukeworks.tech",
    message: concat!(
        r#"Have a question or want to work together? Drop me a line: "#,
        r#"<a href="mailto:me@lukeworks.tech" title="email me@lukeworks.tech">"#,
        r#"me@lukeworks.tech</a>."#
    ),
};

/// 404 Not Found static copy.
pub static NOT_FOUND: Lazy<Error> = Lazy::new(|| Error {
    heading: "$ 404 Segmentation Fault",
    message: "The page you're looking for is out of bounds.",
    action: || {
        view! {
            "Check that the url is correct, file a "
            <a href="mailto:me@lukeworks.tech" title="email bug report to me@lukeworks.tech">bug report</a>
            "or head back to the "
            <a href="/" title="lukeworks.tech homepage">homepage</a>
            "."
        }
    },
});

/// 500 Internal Server Error static copy.
pub static SERVER_ERROR: Lazy<Error> = Lazy::new(|| Error {
    heading: "$ 500 Fatal Exception",
    message: "Something blew up internally.",
    action: || {
        view! {
            "Try again later, file a "
                <a href="mailto:me@lukeworks.tech" title="email bug report to me@lukeworks.tech">bug report</a>
            "or head back to the "
            <a href="/" title="lukeworks.tech homepage">homepage</a>
            "."
        }
    },
});
