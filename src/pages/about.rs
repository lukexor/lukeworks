//! Bio and contact details.

use crate::{components::social_links::ContactCards, lukeworks::SUPPORT_EMAIL};
use leptos::prelude::*;
use leptos_meta::Title;

/// About and contact, sharing one page.
///
/// Contact is a `#contact` section rather than a route of its own, and
/// `content/redirects.toml` sends `/contact` to that fragment.
#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />

        <div class="max-w-3xl">
            <p class="mb-3 font-mono text-[13px] text-primary">"$ cat about.md"</p>
            <h1 class="mb-8 font-mono text-4xl font-bold tracking-tighter">"About"</h1>

            <p class="mb-5 text-lg font-medium leading-relaxed">
                "My name is Lucas Petherbridge. I'm a software engineer and technology enthusiast."
            </p>
            <p class="mb-4 leading-relaxed text-ink-dim">
                "I've been fascinated with computers and technology for as long as I can remember
                and have been coding since age 15 when I first learned about HTML and CSS. I
                primarily use Rust these days, but I've dabbled in just about everything including
                Perl, Ruby, Python, Golang, Java/Kotlin, C/C++, Haskell, and even some 6502
                Assembly!"
            </p>
            <p class="mb-4 leading-relaxed text-ink-dim">
                "I love learning new technologies and finding the best tools for the job. I've
                developed a deep appreciation of problem-solving over the years, with many late
                nights (and cups of coffee!). I am constantly looking for new projects, things to
                learn and better ways to solve problems."
            </p>
            <p class="mb-12 leading-relaxed text-ink-dim">
                "I'm excited about cutting-edge technologies, especially AI and machine learning
                and I'm always on the lookout for anything I can sink my teeth into. My posts here
                share my projects and ideas with the hope they spur discussions about code,
                technology and software practices."
            </p>

            <hr class="mb-10 border-rule" />

            <section id="contact">
                <p class="mb-3 font-mono text-[13px] text-primary">"$ contact --help"</p>
                <h2 class="mb-4 font-mono text-3xl font-bold tracking-tight">"Contact"</h2>
                <p class="mb-7 leading-relaxed text-ink-dim">
                    "Have a question or want to work together? Drop me a line at "
                    <a href=format!("mailto:{SUPPORT_EMAIL}")>{SUPPORT_EMAIL}</a> "."
                </p>
                <ContactCards />
            </section>
        </div>
    }
}
