use color_eyre::Report;

use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{html::push_html, Parser};
use std::{fs, io, path::Path};

fn main() -> Result<(), Report> {
    let html_content = generate_page();
    fs::write("docs/index.html", html_content.into_string()).expect("Failed to write HTML file");
    copy_dir_all("public", "docs")?;
    println!("Built site OK!");
    Ok(())
}

struct Markdown<'a>(&'a str);

impl Markdown<'_> {
    fn render_to(self, output: &mut String) {
        let mut output_html = String::new();
        let parser = Parser::new(self.0);
        push_html(&mut output_html, parser);
        output.push_str(output_html.as_str());
    }
}

fn blog_preview() -> Markup {
    println!("{:?}", std::env::current_dir());

    let paths = fs::read_dir(std::env::current_dir().unwrap().join("src/content")).unwrap();
    println!("{:?}", paths);

    let md_paths: Vec<_> = paths
        .filter_map(Result::ok) // Filter out any Err results from read_dir
        .filter(|entry| {
            if let Ok(file_type) = entry.file_type() {
                file_type.is_file()
            } else {
                false
            }
        })
        .filter(|entry| {
            if let Some(extension) = entry.path().extension() {
                extension == "md"
            } else {
                false
            }
        })
        .collect();

    println!("{:?}", md_paths);
    let local_contents: Vec<String> = md_paths
        .into_iter()
        .map(|x| {
            println!("{:?}", x);
            let path = std::env::current_dir()
                .unwrap()
                .join("src/content/")
                .join(x.file_name().into_string().unwrap());
            // println!("{:?}", path);
            let content = fs::read_to_string(&path).expect("Unable to read file");
            let mut html = "".to_string();
            Markdown(&content).render_to(&mut html);
            html.to_string().clone()
        })
        .collect();

    // TODO parse toml header metadata
    // TODO
    // {{ miku(content="Hit the GRTTy!", miku_img="miku_jump") }}
    // <div style="float: right; justify-content: right; align-items: center;">
    //     <blockquote>
    //         <p>{{content}}</p>
    //     </blockquote>
    //     <img style="max-width: 75%;" src="/miku/{{miku_img}}.png" />
    // </div>

    html!(
        h2 class="slogan" { "/blog" }
        div {
            @for html in local_contents.iter() {
                li.item {
                    div { (PreEscaped(html)) }
                }
            }
        }
    )
}

// https://stackoverflow.com/a/65192210/26371953
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.as_ref().join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir_all(entry_path, target_path)?;
        } else if should_copy_file(&entry_path, &target_path)? {
            fs::copy(&entry_path, &target_path)?;
        }
    }
    Ok(())
}

fn should_copy_file(src: &Path, dst: &Path) -> io::Result<bool> {
    match fs::metadata(dst) {
        Ok(dst_metadata) => {
            let src_metadata = fs::metadata(src)?;
            let src_modified = src_metadata.modified()?;
            let dst_modified = dst_metadata.modified()?;

            // Only copy if source file is newer than destination file
            Ok(src_modified > dst_modified)
        }
        Err(_) => {
            // Destination file doesn't exist, so we need to copy
            Ok(true)
        }
    }
}

fn generate_page() -> Markup {
    template(
        html! {
            (blog_preview())
        }
        .render()
        .into_string(),
    )
}

fn footer() -> Markup {
    html! {
        br;
        br;
        br;
        br;
        p .text-xs { "Made with <3 in 2025" }
    }
}

fn template(inner: String) -> Markup {
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
                     div style="position: absolute; right: 5vw;" {
                         img style="max-width: 35%; float:right;" src="miku_hello.png";
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
