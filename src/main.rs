use color_eyre::Report;

use crate::template::template;
use handlebars::Handlebars;
use mastodon_comments::mastodon_comments;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{html::push_html, Parser};
use std::{
    error::Error, fs::{self, DirEntry, FileType}, io, path::Path
};
use toml::Table;

mod mastodon_comments;
mod template;


use std::fs;
use std::path::Path;


fn generate_pages() -> Result<Vec<(String, Markup)>, Box<dyn Error>> {
    println!("{:?}", std::env::current_dir());

    let paths = fs::read_dir(std::env::current_dir().unwrap().join("src/content")).unwrap();
    println!("{:?}", paths);

    let md_paths: Vec<DirEntry> = paths
        .filter_map(|res| match res {
            Ok(de) => match de.file_type() {
                Ok(ft) => (ft.is_file() && de.path().extension().is_some_and(|ext| ext == "md"))
                    .then_some(Ok(de)),
                Err(e) => Some(Err(e)),
            },
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<_, _>>()?;

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
            fs::read_to_string(&path).expect("Unable to read file")
        })
        .collect();

    let header_contents: Vec<(Table, String)> =
        local_contents.into_iter().map(get_header).collect();

    let header_and_rendered_no_template: Vec<(Table, String)> = header_contents
        .into_iter()
        .map(|(h, c)| {
            let mut html = String::new();
            Markdown(&c).render_to(&mut html);
            let mut out = String::new();
            BlogPost(&html).render_to(&h, &mut out);
            (h, out.to_string().clone())
        })
        .collect();

    // TODO github repo and github pages and github actions ci

    let reg = Handlebars::new();

    let rendered: Vec<String> = header_and_rendered_no_template
        .into_iter()
        .map(|(t, c)| reg.render_template(&c, &t))
        .filter_map(Result::ok) // Filter out any Err results from read_dir
        .collect();

    
    // GRTTy.md -> GRTTy.html content: rendered[0]

    // // (
    // //     ,html!(
    // //     h2 class="slogan" { "/blog" }
    // //     div {
    // //         @for html in rendered.iter() {
    // //             div { (PreEscaped(html)) }
    // //         }
    // //     }
    // ))

    Ok(Vec::new())
}

fn main() -> Result<(), Report> {
    let page_files = generate_pages();

    let mut files = vec![
        ("docs/about.html", about().into_string()),
        (
            "docs/index.html",
            template(
                html! {
                    "temp content"
                }
                .render()
                .into_string(),
            )
            .into_string(),
        ),
    ];

    for (path, content) in files {
        fs::write(path, content).expect("Failed to write HTML file");
    }
    copy_dir_all("public", "docs")?;
    println!("Built site OK!");
    Ok(())
}

fn about() -> Markup {
    template(
        html! {
            p {
                "Made with "
                a href="https://www.getzola.org/" {
                    "ZOLA"
                }
            }
            p {
                "Comments via "
                a href="https://mastodon.social" {
                    "Mastodon"
                }
            }
            p {
                a href="https://www.sigstick.com/pack/Ds8XSC82a5s1iBSx4Bhi" {
                    "Miku-Stickers"
                }
                " by "
                a href="https://www.sigstick.com/stickers?author=kal%20(store-KP-girl))" {
                    "kal (store-KP-girl)"
                }
            }
            p {
                "Made with "
                span class="emoji" {
                    "💕"
                }
                " in Germany"
            }
            a href="https://brainmade.org/" {
                img src="https://brainmade.org/white-logo.svg" alt="Brainmade";
            }
        }
        .render()
        .into_string(),
    )
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

struct BlogPost<'a>(&'a str);

impl BlogPost<'_> {
    fn render_to(self, config: &Table, output: &mut String) {
        let html_template = html! (
            div style="float:right; max-width: 25%;" {
                blockquote {
                    p {
                        "{{extra.miku_q}}"
                    }
                }
                img style="" src="{{extra.miku_img}}.png" {}
            }
        )
        .into_string();

        println!("html_template {:?}", &html_template);

        let html = Handlebars::new()
            .render_template(&html_template, &config)
            .unwrap();

        let comments = mastodon_comments(config).into_string();

        output.push_str(&html);
        println!("push str html {:?}", &html);
        output.push_str(self.0);
        println!("push str 0 {:?}", self.0);
        output.push_str(&comments);
        println!("push str comments {:?}", &comments);
    }
}

fn get_header(content: String) -> (Table, String) {
    let start_delimiter = "+++";
    let end_delimiter = "+++";

    if let Some(start_index) = content.find(start_delimiter) {
        let toml_start = start_index + start_delimiter.len();
        if let Some(end_index) = content[toml_start..].find(end_delimiter) {
            let toml_end = toml_start + end_index;
            let toml_str = &content[toml_start..toml_end];

            println!("converting {:?}", toml_str);
            let toml: Table = toml::from_str(toml_str).unwrap();
            println!("toml {:?}", &toml);

            let markdown_start = toml_end + end_delimiter.len();
            let markdown_content = content[markdown_start..].to_string();

            (toml, markdown_content)
        } else {
            (Table::new(), content) // Handle missing end delimiter
        }
    } else {
        (Table::new(), content) // Handle missing start delimiter
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
            fs::read_to_string(&path).expect("Unable to read file")
        })
        .collect();

    let header_contents: Vec<(Table, String)> =
        local_contents.into_iter().map(get_header).collect();

    let header_and_rendered_no_template: Vec<(Table, String)> = header_contents
        .into_iter()
        .map(|(h, c)| {
            let mut html = String::new();
            Markdown(&c).render_to(&mut html);
            let mut out = String::new();
            BlogPost(&html).render_to(&h, &mut out);
            (h, out.to_string().clone())
        })
        .collect();

    // TODO github repo and github pages and github actions ci

    let reg = Handlebars::new();

    let rendered: Vec<String> = header_and_rendered_no_template
        .into_iter()
        .map(|(t, c)| reg.render_template(&c, &t))
        .filter_map(Result::ok) // Filter out any Err results from read_dir
        .collect();

    html!(
        h2 class="slogan" { "/blog" }
        div {
            @for html in rendered.iter() {
                div { (PreEscaped(html)) }
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
