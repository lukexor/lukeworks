//! Portfolio static data.

use crate::portfolio::models::{BlogPost, ProjectPost};
use once_cell::sync::Lazy;
use std::fs;

pub mod colors {
    /// Dark theme. Should match `tailwind.config.js` and `site.webmanifest`.
    pub const DARK_THEME: &str = "#131821";
    /// Light theme.
    pub const LIGHT_THEME: &str = "#e5e9e1";
}

/// `<Route />` URLs.
pub mod routes {
    pub const HOME: &str = "/";
    pub const HOME_BLOG: &str = "/#blog";
    pub const HOME_PROJECTS: &str = "/#projects";
    pub const HOME_ABOUT: &str = "/#about";
    pub const HOME_CONTACT: &str = "/#contact";

    pub const BLOG: &str = "/blog";
    pub const PROJECTS: &str = "/projects";
    pub const TETANES_WEB: &str = "/tetanes-web";
    pub const POST: &str = "/:postSlug";
    pub const RESUME_PDF: &str = "/resume.pdf";
    pub const RSS: &str = "/rss.xml";
}

/// External Links.
pub mod links {
    pub const GITHUB: &str = "http://github.com/lukexor";
    pub const LINKEDIN: &str = "https://linkedin.com/in/lucaspetherbridge";
    pub const EMAIL: &str = "mailto:me@lukeworks.tech";
}

/// Layout constants.
pub mod layout {
    pub const LOGO: &str = "L";

    pub mod menu {
        pub const HOME: &str = "Home";
        pub const BLOG: &str = "Blog";
        pub const PROJECTS: &str = "Projects";
        pub const ARCHIVE: &str = "Project Archive";
        pub const ABOUT: &str = "About";
        pub const CONTACT: &str = "Contact";
        pub const RESUME: &str = "Resume";
        pub const TETANES_WEB: &str = "TetaNES Web";
    }

    pub mod icons {
        pub const GITHUB: &str = "GitHub - Lucas Petherbridge";
        pub const LINKEDIN: &str = "LinkedIn - Lucas Petherbridge";
        pub const RSS: &str = "RSS Feed - https://lukeworks.tech";
        pub const EMAIL: &str = "Email me@lukeworks.tech";
        pub const DARK_MODE: &str = "Toggle Dark Mode";
    }

    pub mod search {
        pub const PLACEHOLDER: &str = "Search...";
        pub fn result_summary(count: impl Fn() -> usize, query: impl Fn() -> String) -> String {
            let count = count();
            let query = query();
            match count {
                0 => format!("No results for: `{query}`"),
                1 => format!("1 result for: `{query}`"),
                _ => format!("{count} results for: `{query}`"),
            }
        }
    }

    pub mod not_found {
        pub const HEADING: &str = "$ 404 Segmentation Fault";
        pub const BODY: &str = include_str!("../../data/not_found.html");
    }

    pub mod server_error {
        pub const HEADING: &str = "$ 500 Fatal Exception";
        pub const BODY: &str = include_str!("../../data/server_error.html");
    }

    pub fn footer(year: i32, author: &'static str) -> String {
        format!("© {year} {author}. All Rights Reserved.")
    }
}

/// Meta constants.
pub mod meta {
    pub const AUTHOR: &str = "Lucas Petherbridge";
    pub const EMAIL: &str = "me@lukeworks.tech";
    pub const TITLE: &str = "Lucas Petherbridge | Software Engineer";
    pub const ORIGIN: &str = "https://lukeworks.tech";
    pub const DESC: &str = "A blog and project portfolio by Lucas Petherbridge on programming, technology, and video game topics.";
    pub const KEYWORDS: &str =
        "blog, programming, software, technology, video games, web design, web development";
}

/// homepage constants.
pub mod homepage {
    pub mod intro {
        pub const TITLE: &str = r#"Hi, I'm <span class="text-red-400"/>Luke</em>."#;
        pub const SUBTITLE: &str = "Software Engineer. Designer. Thinker.";
        pub const ABOUT: &str = concat!(
            r#"I'm a polyglot software engineer with a long history of solving technical problems "#,
            r#"and designing exceptional solutions. I'm currently focused on improving the studio "#,
            r#"pipeline at <a href="https://www.laika.com/" title="LAIKA, LLC">LAIKA, LLC</a>."#
        );
        pub const ACTION: &str = "Have a look around!";
        pub const EXPLORE: &str = "Explore";
    }

    pub mod about {
        pub const HEADING: &str = "About";
        pub const BODY: &str = include_str!("../../data/about.html");
    }

    pub mod contact {
        pub const HEADING: &str = "Contact";
        pub const BODY: &str = include_str!("../../data/contact.html");
    }
}

/// Blog posts.
// TODO: descriptions, Image sizes, tags, move images to markdown
pub static BLOG_POSTS: Lazy<Vec<BlogPost>> = Lazy::new(|| {
    // let post = fs::read_to_string("../../data/posts/lost-and-found-part-1.md").unwrap();
    // let parser = pulldown_cmark::Parser::new(&post);
    // let mut html_output = String::new();
    // pulldown_cmark::html::push_html(&mut html_output, parser);
    vec![]
    //     vec![
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("491d14d0-69c5-4a5a-b9b2-75337869893d"),
    //                 slug: "tetanes-part-2",
    //                 title: "NES Emulation in Rust: Designs and Frustrations",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Nintendo Entertainment System console",
    //                     200,
    //                     200,
    //                     Url("/images/blog/nes_console.webp"),
    //                 ),
    //                 category: "programming",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/nes_console.webp"),
    //             //     Nintendo Entertainment System console",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 43,
    //             published: Some(DateStr("2020-01-31 21:19:14.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("4dec2631-5a2e-4595-bd51-3753aa572a95"),
    //                 slug: "tetanes-part-1",
    //                 title: "Programming an NES Emulator from Scratch in Rust",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "TetaNES Logo",
    //                     200,
    //                     200,
    //                     Url("/images/blog/tetanes.webp"),
    //                 ),
    //                 category: "Programming",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/tetanes.webp"),
    //             //     TetaNES Logo",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 10,
    //             published: Some(DateStr("2019-09-19 06:10:44.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("1e4c03f7-13fb-4f1e-9468-871f420eb7dd"),
    //                 slug: "software-malaise",
    //                 title: "Software Malaise",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Empty derelict room with a single chair",
    //                     200,
    //                     200,
    //                     Url("/images/blog/software_malaise.webp"),
    //                 ),
    //                 category: "Programming",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/software_malaise.webp"),
    //             //     Empty derelict room with a single chair",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 10,
    //             published: Some(DateStr("2017-03-27 05:36:10.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("47414a25-bd1f-4442-a5b2-b2d17c12cd08"),
    //                 slug: "lost-and-found-part-5",
    //                 title: r#""Lost and Found" : Part 5 - Turbo-charge your career and avoid stagnation"#,
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Lego Stormtrooper on a speeder",
    //                     200,
    //                     200,
    //                     Url("/images/blog/lost_and_found_part_5.webp"),
    //                 ),
    //                 category: "Career",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/lost_and_found_part_5.webp"),
    //             //     Lego Stormtrooper on a speeder",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 6,
    //             published: Some(DateStr("2015-03-18 18:56:01.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("05139fea-394a-44ea-943d-e8b365579ea3"),
    //                 slug: "lost-and-found-part-4",
    //                 title: r#""Lost and Found" : Part 4 - Software development in practice"#,
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Street signs: Success Ln., Failure Dr.",
    //                     200,
    //                     200,
    //                     Url("/images/blog/lost_and_found_part_4.webp"),
    //                 ),
    //                 category: "Career",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/lost_and_found_part_4.webp"),
    //             //     Street signs: Success Ln., Failure Dr.",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 14,
    //             published: Some(DateStr("2015-01-14 22:30:34.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("2aa5e8af-9f75-42d8-a21a-44c2a72dee47"),
    //                 slug: "lost-and-found-part-3",
    //                 title: r#""Lost and Found" : Part 3 - I do not think it means what you think it means"#,
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Inigo Montoya from Princess Bride",
    //                     200,
    //                     200,
    //                     Url("/images/blog/lost_and_found_part_3.webp"),
    //                 ),
    //                 category: "Career",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/lost_and_found_part_3.webp"),
    //             //     Inigo Montoya from Princess Bride",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 9,
    //             published: Some(DateStr("2014-12-18 21:49:53.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("cc149424-f142-4064-8f31-642a48604a54"),
    //                 slug: "lost-and-found-part-2",
    //                 title: r#""Lost and Found" : Part 2 - PC load letter?!"#,
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Broken printer in a field",
    //                     200,
    //                     200,
    //                     Url("/images/blog/lost_and_found_part_2.webp"),
    //                 ),
    //                 category: "Career",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/lost_and_found_part_2.webp"),
    //             //     Broken printer in a field",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 14,
    //             published: Some(DateStr("2014-12-04 03:06:26.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("f1d1209d-9229-4ec0-b5db-feb74500df1b"),
    //                 slug: "lost-and-found-part-1",
    //                 title: r#""Lost and Found" : Part 1 - From Dungeons & Dragons to HTML"#,
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "multi-sided dice",
    //                     200,
    //                     200,
    //                     Url("/images/blog/lost_and_found_part_1.webp"),
    //                 ),
    //                 category: "Career",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/lost_and_found_part_1.webp"),
    //             //     multi-sided dice",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 6,
    //             published: Some(DateStr("2014-11-26 00:52:11.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("231bcc49-3f54-4020-b9b7-713817095a81"),
    //                 slug: "lost-and-found-series",
    //                 title: r#""Lost and Found" Series : Lessons Learned"#,
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "We can draw lessons from the past, but we cannot live in it.",
    //                     200,
    //                     200,
    //                     Url("/images/blog/lost_and_found_part_0.webp"),
    //                 ),
    //                 category: "Career",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/lost_and_found_part_0.webp"),
    //             //     We can draw lessons from the past, but we cannot live in it.",
    //             //     200,
    //             //     200,
    //             // }),
    //             minutes_to_read: 2,
    //             published: Some(DateStr("2014-11-21 07:42:52.000000")),
    //         },
    //         BlogPost {
    //             meta: PostMeta {
    //                 id: uuid!("2181259b-470b-4003-a393-97f831c1f806"),
    //                 slug: "first-post",
    //                 title: "First post",
    //                 description: "",
    //                 thumbnail: ImgAttrs::default(),
    //                 category: "Programming",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             minutes_to_read: 1,
    //             published: Some(DateStr("2014-11-18 00:00:00.000000")),
    //         },
    //     ]
});

// /// Project posts.
pub static PROJECT_POSTS: Lazy<Vec<ProjectPost>> = Lazy::new(|| {
    vec![]
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("03b5d942-a155-4e45-bb85-f00b255130af"),
    //                 slug: "pix-engine",
    //                 title: "PixEngine - A Cross-Platform Graphics Library",
    //                 description: "",
    //                 thumbnail: ImgAttrs::default(),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("https://github.com/lukexor/pix-engine")),
    //             started: DateStr("2019-09-29 23:14:15.000000"),
    //             completed: None,
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("69f027b9-ca8b-4984-b798-f89f2d63004a"),
    //                 slug: "tetanes",
    //                 title: "TetaNES - An NES Emulator written in Rust",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "TetaNES Logo",
    //                     200,
    //                     200,
    //                     Url("/images/blog/tetanes.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/blog/tetanes.webp"),
    //             //     TetaNES Logo",
    //             //     200,
    //             //     200,
    //             // }),
    //             website: Some(Url("https://github.com/lukexor/tetanes")),
    //             started: DateStr("2019-04-01 00:00:00.000000"),
    //             completed: None,
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("a2d94b72-1df6-4c4d-96a9-dc05705d8206"),
    //                 slug: "fluid-simulation",
    //                 title: "Fluid Simulation",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Fluid Simulation",
    //                     200,
    //                     200,
    //                     Url("/images/projects/fluid-simulation.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/fluid-simulation")),
    //             started: DateStr("2020-02-21 23:50:51.000000"),
    //             completed: Some(DateStr("2020-02-21 23:50:52.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("e9b612af-a2e8-4a70-9672-c5e6c47d559b"),
    //                 slug: "fourier",
    //                 title: "Discrete Fourier Transforms",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Discrete Fourier Transform",
    //                     200,
    //                     200,
    //                     Url("/images/projects/fourier.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/fourier")),
    //             started: DateStr("2020-02-12 22:44:58.000000"),
    //             completed: Some(DateStr("2020-02-12 22:44:59.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("24c7d94c-0174-40e9-baf3-cc89d9cee737"),
    //                 slug: "asteroids",
    //                 title: "Asteroids!",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Asteroids",
    //                     200,
    //                     200,
    //                     Url("/images/projects/asteroids.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/asteroids")),
    //             started: DateStr("2020-02-11 20:52:02.000000"),
    //             completed: Some(DateStr("2020-02-11 20:52:05.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("ef06e2ef-8a33-4748-b7f4-81b726ac6e30"),
    //                 slug: "pong",
    //                 title: "Pong",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new("Pong", 200, 200, Url("/images/projects/pong.webp")),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/pong")),
    //             started: DateStr("2020-02-11 02:21:54.000000"),
    //             completed: Some(DateStr("2020-02-11 02:21:56.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("dd286805-b641-4c7c-94c1-9dd758e1edbc"),
    //                 slug: "lorenz-attractor",
    //                 title: "Lorenz Attractor",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Lorenz Attractor",
    //                     200,
    //                     200,
    //                     Url("/images/projects/lorenz-attractor.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/lorenz-attractor")),
    //             started: DateStr("2020-02-06 01:45:16.000000"),
    //             completed: Some(DateStr("2020-02-06 01:45:15.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("12beffea-b5a7-4655-8b3a-11f5f4bbea67"),
    //                 slug: "raycasting-2d",
    //                 title: "2D Raycasting",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "2D Raycasting",
    //                     200,
    //                     200,
    //                     Url("/images/projects/raycasting-2d.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/raycasting-2d")),
    //             started: DateStr("2020-02-06 00:22:09.000000"),
    //             completed: Some(DateStr("2020-02-06 00:22:13.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("5444e624-f889-46b7-bc03-539bf36b608b"),
    //                 slug: "fireworks",
    //                 title: "Fireworks!",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Fireworks",
    //                     200,
    //                     200,
    //                     Url("/images/projects/fireworks.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/fireworks")),
    //             started: DateStr("2020-02-03 12:00:00.000000"),
    //             completed: Some(DateStr("2020-02-05 22:40:57.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("90254ae4-59fe-45eb-9744-ca6779804356"),
    //                 slug: "matrix",
    //                 title: "The Matrix Has You",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "The Matrix",
    //                     200,
    //                     200,
    //                     Url("/images/projects/matrix.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/matrix")),
    //             started: DateStr("2020-01-30 19:59:02.000000"),
    //             completed: Some(DateStr("2020-01-30 19:59:03.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("cc9bdb88-47bb-4342-9c03-bcca9d4b21a5"),
    //                 slug: "maze-astar",
    //                 title: "Maze Generation and A* Search",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Maze A* Search",
    //                     200,
    //                     200,
    //                     Url("/images/projects/maze-astar.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("/sketch/maze-astar")),
    //             started: DateStr("2020-01-28 19:57:34.000000"),
    //             completed: Some(DateStr("2020-01-29 18:16:02.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("e41d3922-6333-4a4a-a5e5-f8fe9e4c4b94"),
    //                 slug: "chatdot",
    //                 title: "ChatDot",
    //                 description: "",
    //                 thumbnail: ImgAttrs::default(),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("https://github.com/lukexor/ChatDot")),
    //             started: DateStr("2017-04-24 02:45:29.000000"),
    //             completed: Some(DateStr("2017-06-19 02:45:43.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("a5109baf-2c7a-442f-9b83-75e3f181f68b"),
    //                 slug: "bell-demodulator",
    //                 title: "A Bell 103 Demodulator",
    //                 description: "",
    //                 thumbnail: ImgAttrs::default(),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: None,
    //             website: Some(Url("https://github.com/lukexor/bell103_demodulator")),
    //             started: DateStr("2019-04-26 02:42:01.000000"),
    //             completed: Some(DateStr("2019-04-26 02:42:04.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("852ce423-8ac4-47e7-a41e-0cc9b7cd7941"),
    //                 slug: "haskelltaire",
    //                 title: "Haskelltaire",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Haskelltaire",
    //                     200,
    //                     200,
    //                     Url("/images/projects/haskelltaire.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/projects/haskelltaire.webp"),
    //             //     Haskelltaire",
    //             //     200,
    //             //     200,
    //             // }),
    //             website: Some(Url("https://github.com/lukexor/haskelltaire")),
    //             started: DateStr("2019-03-01 00:00:00.000000"),
    //             completed: Some(DateStr("2019-03-27 00:00:00.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("8ce4440f-3cee-4c57-8d1c-f0addc91fa34"),
    //                 slug: "umflint-env-sci-club",
    //                 title: "UM-Flint Environmental Science Club Website",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "UM-Flint Environmental Science Club Website",
    //                     200,
    //                     200,
    //                     Url("/images/projects/umflint_env_club.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/projects/umflint_env_club.webp"),
    //             //     UM-Flint Environmental Science Club Website",
    //             //     200,
    //             //     200,
    //             // }),
    //             website: Some(Url("https://umflint-env.lukeworks.tech/")),
    //             started: DateStr("2007-09-01 12:00:00.000000"),
    //             completed: Some(DateStr("2007-11-01 12:00:00.000000")),
    //         },
    //         ProjectPost {
    //             meta: PostMeta {
    //                 id: uuid!("0c985e6c-1569-44d9-a286-0122758a8830"),
    //                 slug: "personal-portfolio",
    //                 title: "Personal Portfolio/Blog",
    //                 description: "",
    //                 thumbnail: ImgAttrs::new(
    //                     "Portfolio Website",
    //                     200,
    //                     200,
    //                     Url("/images/projects/portfolio_blog.webp"),
    //                 ),
    //                 category: "",
    //                 tags: HashSet::new(),
    //             },
    //             // image: Some(ImageAttrs::new(
    //             //     Url("/images/projects/portfolio_blog.webp"),
    //             //     Portfolio Website",
    //             //     200,
    //             //     200,
    //             // }),
    //             website: Some(Url("https://lukeworks.tech/")),
    //             started: DateStr("2014-08-12 16:49:29.000000"),
    //             completed: Some(DateStr("2015-03-18 17:58:41.000000")),
    //         },
    //     ]
});
