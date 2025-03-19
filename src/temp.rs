use maud::{html, Markup};

fn render() -> Markup {
    html! {
        ("<!DOCTYPE html>")
        html lang="en" {
            head {
                link rel="icon" href="favicon.ico";
                script src="https://cdn.tailwindcss.com" {}
                link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Noto+Emoji|Space+Mono|Space+Grotesk|VT323";
                script src="tw.js" {}

                script {
                    (r#"
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
                    "#)
                }

                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title class="text-4xl" { "chuu!" }
            }
            body class="bg-black text-white font-mono text-sm md:text-2xl mx-auto w-full max-w-5xl" {
                div class="flex w-full justify-center" { (logo()) }

                nav class="flex items-center justify-between flex-wrap bg-black-500 p-6" {
                    div class="flex items-center flex-shrink-0 text-white mr-6" {
                        a href="/" class="font-semibold text-xl tracking-tight" { "chuu.dev" }
                    }
                    div style="position: absolute; right: 5vw;" {
                        img style="max-width: 35%; float:right;" src="miku_hello.png";
                    }
                    div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto" {
                        div class="text-xl lg:flex-grow" {
                            a href="about.html" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "about" }
                            a href="https://github.com/chuu-p/chuu.dev" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "github" }
                            a href="https://mastodon.social/@chuu_p" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "mastodon" }
                            a href="mailto:chuu801@pm.me" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4" { "email" }
                        }
                    }
                }

                div class="border-black border-8 container mx-auto" { (inner) }
                (footer())
            }
        }
    }
}
