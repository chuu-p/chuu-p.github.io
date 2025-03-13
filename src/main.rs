use blog::copy_static_content;
use color_eyre::{eyre::OptionExt, Report};

use crate::components::mastodon_comments::mastodon_comments;
use crate::components::template::template;
use handlebars::{Handlebars, RenderError};
use maud::{html, Render};
use pulldown_cmark::{html::push_html, Parser};
use std::{
    fs::{self, DirEntry},
    io::{self, ErrorKind},
    path::Path,
    sync::LazyLock,
};
use toml::Table;

mod components;

static REG: LazyLock<Handlebars<'static>> = LazyLock::new(Handlebars::new);

/// On error handling: This is a program designed to be used via command line by
/// a technical audience. This is why we bubble the errors to the main function
/// and use `color_eyre::Report` to generate an error report on the console.
fn main() -> Result<(), Report> {
    color_eyre::install()?;

    const OUT_DIR: &str = "docs";
    const PUBLIC_DIR: &str = "public";
    const CONTENT_DIR: &str = "src/content";

    let md_posts = read_all_files(CONTENT_DIR, "md")?;
    let html_posts = render_posts_to_html(&md_posts)?;
    save_html_posts(&html_posts, OUT_DIR)?;

    save_file(
        &template(component_index(html_posts)?).into_string(),
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

fn component_index(posts: Vec<(String, Table, String)>) -> Result<String, Report> {
    let links_contents = posts
        .into_iter()
        .map(|(path, config, content)| (format!("/{}", path), config, content))
        .collect::<Vec<(String, Table, String)>>();

    Ok(html! {
        h2 class="slogan" { "/blog" }
        @for (link, config, _content) in links_contents.into_iter() {
            a href=(link) { (link) }
            a { (config.get("title").ok_or_eyre(format!("Config Entry 'title' not present in {}", link))?) }
        }
    }
    .render()
    .into_string())
}

fn save_file(content: &str, out_dir: &str, file_name: &str) -> io::Result<()> {
    let target = Path::new(out_dir).join(file_name);
    fs::write(&target, content)?;
    Ok(())
}

fn read_all_files(path: &str, extension: &str) -> io::Result<Vec<(String, String)>> {
    let cwd = std::env::current_dir()?;

    let entries = fs::read_dir(cwd.join(path))?
        .filter_map(|res| {
            let de = res.ok()?;
            let ft = &de.file_type().ok()?;
            if ft.is_file() && de.path().extension().is_some_and(|ext| ext == extension) {
                Some(Ok::<std::fs::DirEntry, io::Error>(de))
            } else {
                None
            }
        })
        .collect::<Result<Vec<DirEntry>, io::Error>>()?;

    entries
        .into_iter()
        .map(|de| {
            let path = cwd.join("src/content/").join(de.file_name());
            let mut html_path = path.clone();
            html_path.set_extension("html");

            let file_name = html_path
                .file_name()
                .ok_or_else(|| io::Error::new(ErrorKind::Other, "filename error"))?
                .to_str()
                .ok_or_else(|| io::Error::new(ErrorKind::Other, "to_str error"))?
                .to_string();

            let content = fs::read_to_string(&path)?;

            Ok((file_name, content))
        })
        .collect::<Result<Vec<(String, String)>, _>>()
}

fn render_posts_to_html(
    posts: &[(String, String)],
) -> Result<Vec<(String, Table, String)>, Report> {
    let res = posts
        .iter()
        .map(|(path, post)| {
            let (table, content) = get_header(post.to_string())?;

            let html = Markdown(&content).render();
            let out = BlogPost(&html).render(&table);

            let rendered_page = REG.render_template(&out?, &table)?;

            let rendered_page = template(rendered_page).into_string();

            Ok::<(String, Table, String), Report>((path.to_owned(), table, rendered_page))
        })
        .collect::<Result<Vec<(String, Table, String)>, Report>>();

    res
}

fn save_html_posts(posts: &[(String, Table, String)], output_dir: &str) -> io::Result<()> {
    for (path, _config, page) in posts {
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
    fn render(self, config: &Table) -> Result<String, RenderError> {
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
        let mut html = Handlebars::new().render_template(&html_template, &config)?;
        let comments = mastodon_comments(config).into_string();
        html.push_str(self.0);
        html.push_str(&comments);
        Ok(html)
    }
}

// FIXME
fn get_header(content: String) -> Result<(Table, String), Report> {
    let start_delimiter = "+++";
    let end_delimiter = "+++";

    if let Some(start_index) = content.find(start_delimiter) {
        let toml_start = start_index + start_delimiter.len();
        if let Some(end_index) = content[toml_start..].find(end_delimiter) {
            let toml_end = toml_start + end_index;
            let toml_str = &content[toml_start..toml_end];

            println!("converting {:?}", toml_str);
            let toml: Table = toml::from_str(toml_str)?;
            println!("toml {:?}", &toml);

            let markdown_start = toml_end + end_delimiter.len();
            let markdown_content = content[markdown_start..].to_string();

            Ok((toml, markdown_content))
        } else {
            Ok((Table::new(), content)) // Handle missing end delimiter
        }
    } else {
        Ok((Table::new(), content)) // Handle missing start delimiter
    }
}
