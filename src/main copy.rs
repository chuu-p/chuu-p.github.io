use crate::logo::logo;
use color_eyre::Report;
use hypertext::{
    html_elements, maud, maud_move, rsx, GlobalAttributes, RenderIterator, Renderable, Rendered,
};
use lazy_static::lazy_static;
use pulldown_cmark::{html::push_html, Parser};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
mod logo;

use std::{fs, io};

static mut CONTENTS: Vec<(String, String, String)> = vec![];

fn main() -> Result<(), Report> {
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
    let local_contents: Vec<(String, String, String)> = md_paths
        .into_iter()
        .map(|x| {
            let path = std::env::current_dir()
                .unwrap()
                .join("src/content/")
                .join(x.file_name().into_string().unwrap());
            println!("{:?}", path);
            let content = fs::read_to_string(&path).expect("Unable to read file");
            let html = Markdown(&content).render().0;
            (path.into_os_string().into_string().unwrap(), content, html)
        })
        .collect();

    unsafe {
        CONTENTS = local_contents;
    };

    build(vec![
        ("docs/index.html", index().render()),
        ("docs/credits.html", credits_page().render()),
    ])?;
    copy_dir_all("public", "docs")?;
    println!("Built site OK!");
    Ok(())
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

fn index() -> impl Renderable {
    page(
        heading(),
        // intro(), sect1(), sect2(), about()
    )
}

fn credits_page() -> impl Renderable {
    let _ = rsx! {
        <h2 class="text-2xl" > <b>Credits</b> </h2>
        <ul class="list-disc">
            <li>
                "Logo based on \"Human\" by JunGSa from "
                <a class="underline" href="https://thenounproject.com/browse/icons/term/human/" target="_blank" title="Human Icons">Noun Project</a>
            </li>
            <li>
                "And \"seed\" by Adrian Syauqi from "
            <a class="underline" href="https://thenounproject.com/browse/icons/term/seed/" target="_blank" title="seed Icons">Noun Project</a>
            </li>
        </ul>
        <br/>
        "Special thanks to everyone who workshopped the logo with me, especially super patron supporter Andrew Jackson. Andrew, I should be paying YOU!"
    };

    template(maud! {
        (heading())
        h2.text-3xl { "Credits" }
        ul {
            li { "Logo based on \"Human\" by JunGSa from "
                 a.underline href="https://thenounproject.com/browse/icons/term/human/" {
                    "Noun Project"
                }
            }
            li {
                "And \"seed\" by Adrian Syauqi from "
                a.underline href="https://thenounproject.com/browse/icons/term/seed/" {
                    "Noun Project"
                }
            }
        }
        br;
        "Special thanks to everyone who workshopped the logo with me, especially super patron supporter Andrew Jackson. Andrew, I should be paying YOU!"
    })
}

#[allow(dead_code)]
struct Markdown<'a>(&'a str);

impl Renderable for Markdown<'_> {
    fn render_to(self, output: &mut String) {
        let mut output_html = String::new();
        let parser = Parser::new(self.0);
        push_html(&mut output_html, parser);
        output.push_str(output_html.as_str());
    }
}

fn template(inner: impl Renderable) -> impl Renderable {
    rsx! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <link
                    href="favicon.ico"
                    rel="icon">
                <script src="https://cdn.tailwindcss.com"></script>
                <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Noto+Emoji|Space+Mono|Space+Grotesk|VT323">
                <script src="tw.js"></script>

            <script>
                r#" tailwind.config = {
                    theme: {
                        container: {
                            center: true,
                        },
                        fontFamily: {
                            "mono": "Space Mono, monospace",
                            "sans": Space Grotesk, sans-serif",
                        }
                    }
                }"#
            </script>

            <meta charset="utf-8"/>

            <meta content="width=device-width, initial-scale=1" name="viewport"/>
            <title class="text-4xl">"chuu!"</title>

            </head>

                <body class="bg-black text-white font-mono text-sm md:text-2xl mx-auto w-full max-w-5xl">

                <div class="flex w-full justify-center">
                    { logo() }
                </div>

                    <nav class="flex items-center justify-between flex-wrap bg-black-500 p-6">
                        <div class="flex items-center flex-shrink-0 text-white mr-6">
                            <a href="/" class="font-semibold text-xl tracking-tight">chuu.dev</a>
                        </div>
                        <div style="position: absolute; right: 5vw;">
                            <img style="max-width: 35%; float:right;" src="miku_hello.png" />
                        </div>
                        <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                            <div class="text-xl lg:flex-grow">
                                <a href="about.html" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    about
                                </a>
                                <a href="https://github.com/chuu-p/chuu.dev" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    github
                                </a>
                                <a href="https://mastodon.social/@chuu_p" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    mastodon
                                </a>
                                <a href="mailto:j@chuu.dev" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    email
                                </a>
                            </div>
                        </div>
                    </nav>

                    <div class="border-black border-8 container mx-auto">
                        {inner}
                    </div>

            { footer() }

            </body>
        </html>
    }
}

/// NOTE: the widget requires https to load
fn widget() -> impl Renderable {
    rsx! {
        <img class="w-1/2" src="video.png" />
    }
}

fn heading() -> impl Fn(&mut String) {
    let heading = rsx! {
        <h2 class="slogan"><b class="text-2xl" > "/blog" </b></h2>
        <br/>
    };
    heading
}

fn blog_preview(contents: Vec<(String, String, String)>) -> impl Fn(&mut String) {
    println!("{:?}", contents);

    maud_move! {
        div {
            @for (path, content, html) in contents.iter() {
                li.item {
                    div { (
                        rsx ! {
                            dangerous_inner_html: "{html}"
                        }

                        ) }
                    // div { "path " (path) }
                    // div { "content " (content) }
                    // div { "html " (html) }
                }
            }
        }
    }
}

fn about() -> impl Fn(&mut String) {
    rsx! {

        <h2 id="about" class="text-4xl"><b>About Me</b></h2>
        <br/>
        "I'm Tris, I'm a writer and producer of "<a class="underline" href="http://noboilerplate.org">"fast, technical videos"</a>", and "<a class="underline" href="https://namtao.com">"audiofiction and music."</a>
        <br/>
        "My first career was as a web developer, doing production on the side for 15 years, but in 2022 I accidentally become entirely self-employed thanks to the surprising success of my YouTube channel, No Boilerplate."
        <br/>
        <br/>
        "At heart I'm still a software developer, I'll re-use 100 libraries to avoid writing 10 lines of code - standing on the shoulders of giants is the only way I know how I get around."
        <br/>
        "But I've looked for a way to mark my videos and stories as being made by humans, not AI, and I can't find one that works in exactly the way I want."
        <br/>
        "I don't want something that says 'NO AI USED', signposts that are negative and judgemental, nor a '100% human made' guarantee - what would that even MEAN these days?"
        <br/>
        "I want a positive mark."
        <br/>
        <br/>
        "I have many issues with the options I've seen so far, from having multiple logos (which is confusing) to the fixation on AI being inherently evil (this will not always be the case)."
        <br/>
        "My root concern with these methods is that they are negative. `AI = bad`.
        But I think the correct way to present this is `human = good`."
        <br/>
    }
}

fn footer() -> impl Renderable {
    let footer = rsx! {
        <br/>
        <br/>
        <br/>
        <br/>
        <p class="text-xs">"Made with <3 in 2025"</p>
    };
    footer
}

fn page(heading: impl Renderable) -> impl Renderable {
    let contents;
    unsafe {
        contents = CONTENTS.clone();
    }

    template(rsx! {
        { heading }
        { blog_preview(contents) }
    })
}

fn build(pages: Vec<(&str, Rendered<String>)>) -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    for (page, fun) in pages {
        let output = fun.into_inner();
        std::fs::write(page, output)?;
    }
    Ok(())
}
