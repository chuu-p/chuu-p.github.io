use blog::copy_static_content;
use color_eyre::Report;

use crate::components::mastodon_comments::mastodon_comments;
use crate::components::template::template;
use core::fmt;
use handlebars::{Handlebars, RenderError};
use maud::{html, Markup, PreEscaped, Render};
use pulldown_cmark::{html::push_html, Parser};
use std::{
    error::Error,
    fmt::format,
    fs::{self},
    io,
    path::Path,
    sync::LazyLock,
};
use toml::Table;

mod components;

static REG: LazyLock<Handlebars<'static>> = LazyLock::new(Handlebars::new);

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    // lets write some testable code!

    const OUT_DIR: &str = "docs";
    const PUBLIC_DIR: &str = "public";
    const CONTENT_DIR: &str = "src/content";

    let md_posts = read_all_files(CONTENT_DIR, "md")?;
    let html_posts = render_posts_to_html(&md_posts)?;
    save_html_posts(&html_posts, OUT_DIR)?;

    save_file(
        &template(component_index(html_posts)).into_string(),
        OUT_DIR,
        "index.html",
    )?;
    save_file(
        &template(component_about()).into_string(),
        OUT_DIR,
        "about.html",
    )?;

    copy_static_content(Path::new(PUBLIC_DIR), Path::new(OUT_DIR))?;

    println!("Built site OK!");
    Ok(())
}

fn component_index(posts: Vec<(String, String)>) -> String {
    let links_contents = posts
        .into_iter()
        .map(|(path, content)| (format!("/{}", path), content))
        .collect::<Vec<(String, String)>>();

    html! {
        h2 class="slogan" { "/blog" }
        @for (link, content) in links_contents.into_iter() {
            a href=(link) { (link) }
        }
    }
    .render()
    .into_string()
}

fn save_file(content: &str, out_dir: &str, file_name: &str) -> io::Result<()> {
    let target = Path::new(out_dir).join(file_name);
    fs::write(&target, content)?;
    Ok(())
}

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn read_all_files(path: &str, extension: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let cwd = std::env::current_dir()?;

    let entries: Vec<_> = fs::read_dir(cwd.join(path))
        .unwrap()
        .filter_map(|res| {
            res.and_then(|de| {
                de.file_type().map(|ft| {
                    (ft.is_file() && de.path().extension().is_some_and(|ext| ext == extension))
                        .then_some(de)
                })
            })
            .transpose()
        })
        .collect::<Result<_, _>>()?;

    Ok(entries
        .into_iter()
        .map(|de| {
            let path = cwd.join("src/content/").join(de.file_name());
            let mut html_path = path.clone();
            html_path.set_extension("html");
            Ok::<(String, String), Box<dyn Error>>((
                String::from(
                    html_path.file_name()
                        .ok_or_else(|| MyError::new("filname error"))?
                        .to_str()
                        .ok_or_else(|| MyError::new("to_str error"))?,
                ),
                fs::read_to_string(&path)?,
            ))
        })
        .map(|x| {
            println!("unwrapping {:?}", x);
            x.unwrap()
        })
        // .filter_map(Result::ok)
        .collect::<Vec<(String, String)>>())
}

fn render_posts_to_html(posts: &[(String, String)]) -> Result<Vec<(String, String)>, RenderError> {
    let res = posts
        .iter()
        .map(|(path, post)| {
            let (table, content) = get_header(post.to_string());

            let html = Markdown(&content).render();
            let out = BlogPost(&html).render(&table);

            let rendered_page = REG.render_template(&out, &table)?;

            let rendered_page = template(rendered_page).into_string();

            Ok::<(String, String), Box<dyn Error>>((path.to_owned(), rendered_page))
        })
        .filter_map(Result::ok)
        .collect::<Vec<(String, String)>>();

    Ok(res)
}

fn save_html_posts(posts: &[(String, String)], output_dir: &str) -> io::Result<()> {
    for (path, page) in posts {
        let mut target = Path::new(output_dir).join(path);
        target.set_extension("html");
        fs::write(&target, page)?;
    }
    Ok(())
}

fn component_about() -> String {
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
    .render()
    .into_string()
}

struct Markdown<'a>(&'a str);

impl Markdown<'_> {
    fn render(self) -> String {
        let mut output_html = String::new();
        let parser = Parser::new(self.0);
        push_html(&mut output_html, parser);
        output_html
    }
}

struct BlogPost<'a>(&'a str);

impl BlogPost<'_> {
    fn render(self, config: &Table) -> String {
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
        let mut html = Handlebars::new()
            .render_template(&html_template, &config)
            .unwrap();
        let comments = mastodon_comments(config).into_string();
        html.push_str(self.0);
        html.push_str(&comments);
        html
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
