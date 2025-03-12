use blog::copy_static_content;
use color_eyre::Report;

use crate::components::mastodon_comments::mastodon_comments;
use crate::components::template::template;
use handlebars::{Handlebars, RenderError};
use maud::{html, Markup, PreEscaped, Render};
use pulldown_cmark::{html::push_html, Parser};
use std::{
    error::Error,
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
        &template(index(html_posts)).into_string(),
        OUT_DIR,
        "index.html",
    )?;
    save_file(&template(about()).into_string(), OUT_DIR, "about.html")?;

    copy_static_content(Path::new(PUBLIC_DIR), Path::new(OUT_DIR))?;

    println!("Built site OK!");
    Ok(())
}

fn index(posts: Vec<(String, String)>) -> String {
    // let links  = posts.iter().map(|(path, content)| {

    // }).collect::<String>();
    
    html! {
        h2 class="slogan" { "/blog" }
        (format!("temp content posts: {}", posts.len()))
        // @for (path, ) in rendered.iter() {
        //     div { (PreEscaped(html)) }
        // }
    }
    .render()
    .into_string()
}

fn save_file(content: &str, out_dir: &str, file_name: &str) -> io::Result<()> {
    let target = Path::new(out_dir).join(file_name);
    fs::write(&target, content)?;
    Ok(())
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
            (
                String::from(path.file_name().unwrap().to_str().unwrap()),
                fs::read_to_string(&path).unwrap(),
            )
        })
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

fn about() -> String {
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
            let html = Markdown(&c).render();
            let out = BlogPost(&html).render(&h);
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

fn generate_pages() -> Result<Vec<String>, Report> {
    static REG: LazyLock<Handlebars<'static>> = LazyLock::new(Handlebars::new);
    let cwd = std::env::current_dir()?;

    let entries: Vec<_> = fs::read_dir(cwd.join("src/content"))
        .unwrap()
        // read paths
        .filter_map(|res| {
            res.and_then(|de| {
                de.file_type().map(|ft| {
                    (ft.is_file() && de.path().extension().is_some_and(|ext| ext == "md"))
                        .then_some(de)
                })
            })
            .transpose()
        })
        .collect::<Result<_, _>>()?;

    let mut file_paths_html = Vec::new();

    // read contents
    for de in entries {
        let path = cwd.join("src/content/").join(de.file_name());

        let (table, content) = get_header(fs::read_to_string(&path)?);

        let html = Markdown(&content).render();
        let out = BlogPost(&html).render(&table);

        let rendered_page = REG.render_template(&out, &table)?;

        let rendered_page = template(rendered_page).into_string();

        let mut target = Path::new("docs").join(path.file_name().unwrap());

        target.set_extension("html");

        fs::write(&target, rendered_page)?;

        file_paths_html.push(target.to_str().unwrap().to_string().clone());
    }

    Ok(file_paths_html)
}
