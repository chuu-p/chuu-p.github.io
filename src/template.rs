use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::fs;

fn footer() -> Markup {
    html! {
        br;
        br;
        br;
        br;
        p .text-xs { "Made with <3 in 2025" }
    }
}

pub fn template(inner: String) -> Markup {
    let cwd = std::env::current_dir().unwrap();
    let logo = fs::read_to_string(cwd.join("src/logo.html")).expect("Unable to read file");
    let style = fs::read_to_string(cwd.join("src/style.css")).expect("Unable to read file");

    html! {
    (DOCTYPE)
    html lang="en" {

        head {
            link rel="icon" href="favicon.ico";
            script src="https://cdn.tailwindcss.com" {}
            link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Noto+Emoji|Space+Mono|Space+Grotesk|VT323";
            script src="tw.js" {}

            script {
                (PreEscaped(r#"
                    tailwind.config = {
                        theme: {
                            container: {
                                center: true,
                            },
                            fontFamily: {
                                "mono": "Space Mono, monospace",
                                "sans": "Space Grotesk, sans-serif",
                            }
                        }
                    }
                    "#))
            }

            // https://stackoverflow.com/a/77568510/26371953
            style type="text/tailwindcss" {
                (PreEscaped(style))
            }

            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { "chuu!" }

            script src="highlight.min.js" {}
            // TODO download to local
            script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/protobuf.min.js" {}
            style type="text/tailwindcss" {
                (PreEscaped("code:not([class]) { color: #86cecb; }"))
            }
            // link rel="stylesheet" href="styles/github-dark.css";
            link rel="stylesheet" href="styles/ir-black.css";

            script { "hljs.highlightAll();" }


        }
        body class="bg-black text-white font-sans text-l md:text-2xl mx-auto w-full max-w-5xl" {
                div class="flex w-full justify-center" { (PreEscaped(logo)) }

                 nav class="flex items-center justify-between flex-wrap bg-black-500 p-6" {
                     div class="flex items-center flex-shrink-0 text-white mr-6" {
                         a href="/" class="font-semibold text-xl tracking-tight" { "chuu.dev" }
                     }

                     div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto" {
                         div class="text-xl lg:flex-grow" {
                             a href="about.html" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "about" }
                             a href="https://github.com/chuu-p/chuu.dev" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "github" }
                             a href="https://mastodon.social/@chuu_p" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "mastodon" }
                             a href="mailto:j@chuu.dev" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "email" }
                         }
                     }
                 }

                 div class="p-6 container mx-auto" { (
                    PreEscaped(inner)
                ) }
                 (footer())
            }
    }}
}
