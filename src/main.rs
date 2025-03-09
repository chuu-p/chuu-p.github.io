use color_eyre::Report;

use crate::template::template;
use handlebars::Handlebars;
use mastodon_comments::mastodon_comments;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{html::push_html, Parser};
use std::{
    error::Error,
    fs::{self, DirEntry, FileType},
    io,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use toml::Table;

mod mastodon_comments;
mod template;

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

fn main() -> Result<(), Report> {
    let posts = collect_markdown_posts("src/content")?;
    let html_posts = render_posts_to_html(&posts)?; // TODO
    save_html_posts(&html_posts, "docs")?;

    render_index_page(&html_posts, "docs/index.html")?;
    render_about_page("src/about.md", "docs/about.html")?;

    copy_static_content("public", "docs")?;

    println!("Built site OK!");
    Ok(())

    // generate_pages()
    // let page_files = generate_pages();

    // let mut files = vec![
    //     ("docs/about.html", about().into_string()),
    //     (
    //         "docs/index.html",
    //         template(
    //             html! {
    //                 "temp content"
    //             }
    //             .render()
    //             .into_string(),
    //         )
    //         .into_string(),
    //     ),
    // ];

    // for (path, content) in files {
    //     fs::write(path, content).expect("Failed to write HTML file");
    // }
    // copy_dir_all("public", "docs")?;
    // println!("Built site OK!");
    // Ok(())
}

fn collect_markdown_posts(path: &str) -> Result<Vec<String>, Report> {
    // Implementation later
    todo!()
}

fn render_posts_to_html(posts: &[String]) -> Result<Vec<(String, String)>, Report> {
    // Implementation later
    todo!()
}

fn save_html_posts(posts: &[(String, String)], output_dir: &str) -> Result<(), Report> {
    // Implementation later
    todo!()
}

fn render_index_page(posts: &[(String, String)], output_path: &str) -> Result<(), Report> {
    // Implementation later
    todo!()
}

fn render_about_page(input_path: &str, output_path: &str) -> Result<(), Report> {
    // Implementation later
    todo!()
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

/// Recursively copies static content from a source directory to a destination directory.
///
/// This function first creates the destination directory if it does not exist.
/// Then, it iterates through each entry in the source directory. If an entry is a directory,
/// the function recursively calls itself to copy the contents of that directory.
/// If an entry is a file, the function checks if the file should be copied using the
/// `should_copy_file` function. If `should_copy_file` returns `true`, the file is copied
/// from the source to the destination.
///
/// # Arguments
///
/// * `src` - A string slice representing the path to the source directory.
/// * `dst` - A string slice representing the path to the destination directory.
///
/// # Returns
///
/// * `Ok(())` if the copy operation was successful.
/// * `Err(io::Error)` if an error occurred during the copy operation, such as if a directory
///   cannot be created or read, or if a file cannot be copied.
///
/// # Errors
///
/// This function can return an `io::Error` if any of the following occur:
///
/// * The source directory cannot be read.
/// * A directory cannot be created.
/// * A file cannot be copied.
/// * Metadata of a file cannot be accessed.
///
/// # Panics
///
/// This function panics if `entry_path.to_str()` or `target_path.to_str()` return `None`.
/// This can happen if the path contains invalid Unicode.
// https://stackoverflow.com/a/65192210/26371953
fn copy_static_content(src: &str, dst: &str) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = Path::new(dst).join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_static_content(entry_path.to_str().unwrap(), target_path.to_str().unwrap())?;
        } else if should_copy_file(&entry_path, &target_path)? {
            fs::copy(&entry_path, &target_path)?;
        }
    }
    Ok(())
}

/// Determines whether a file should be copied from source to destination.
///
/// The function checks if the destination file exists. If it does not exist,
/// the function returns `Ok(true)`, indicating that the file should be copied.
/// If the destination file exists, the function compares the modification times
/// of the source and destination files. If the source file is newer than the
/// destination file, the function returns `Ok(true)`. Otherwise, it returns
/// `Ok(false)`.
///
/// # Arguments
///
/// * `src` - A reference to a `Path` representing the source file.
/// * `dst` - A reference to a `Path` representing the destination file.
///
/// # Returns
///
/// * `Ok(true)` if the file should be copied.
/// * `Ok(false)` if the file should not be copied.
/// * `Err(io::Error)` if an error occurred during metadata retrieval.
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
