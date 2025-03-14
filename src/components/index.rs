use color_eyre::Report;
use maud::{html, Render};

use crate::BlogPostConfig;

pub fn index(posts: Vec<(String, BlogPostConfig, String)>) -> Result<String, Report> {
    let mut links_contents = posts
        .into_iter()
        .map(|(path, config, content)| (format!("/{}", path), config, content))
        .collect::<Vec<(String, BlogPostConfig, String)>>();
    links_contents.sort_by(|a, b| a.1.date.cmp(&b.1.date));
    links_contents.reverse();

    Ok(html! {
        h2 class="slogan" { "/blog" }
        table class="sortable border-none" {
            thead class="border-none" {
                tr class="border-none" {
                    th class="border-none" { "Category"}
                    th class="border-none" { "Date"}
                    th class="border-none" { "Title"}
                }
            }
            tbody class="border-none" {
                @for (link, config, _content) in links_contents.into_iter() {
                    tr class="border-none" {
                        td class="border-none" {
                            div { "blog" }
                        }
                        td class="border-none" {
                            div { (config.date) }
                        }
                        td class="border-none" {
                            a href=(link) { (config.title) }
                        }
                    }
                }
            }
        }
    }
    .render()
    .into_string())
}
