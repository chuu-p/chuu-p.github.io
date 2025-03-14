use std::sync::LazyLock;

use color_eyre::{eyre::OptionExt, Report};
use handlebars::{Handlebars, RenderError};
use log::debug;
use maud::html;
use pulldown_cmark::{html::push_html, Parser};

use crate::BlogPostConfig;

use super::{mastodon_comments::mastodon_comments, template::template};

static REG: LazyLock<Handlebars<'static>> = LazyLock::new(Handlebars::new);

pub fn blog_post(markdown: String) -> Result<(BlogPostConfig, String), Report> {
    let (table, content) = get_header(markdown)?;

    let html = Markdown(&content).render();
    let out = BlogPost(&html).render(&table);

    let rendered_page = REG.render_template(&out?, &table)?;

    let rendered_page = template(rendered_page).into_string();
    Ok((table, rendered_page))
}

fn get_header(content: String) -> Result<(BlogPostConfig, String), Report> {
    let start_delimiter = "+++";
    let end_delimiter = "+++";

    let start_index = content
        .find(start_delimiter)
        .ok_or_eyre(format!("start delimiter {} not found", &start_delimiter))?;
    let toml_start = start_index + start_delimiter.len();
    let end_index = content[toml_start..]
        .find(end_delimiter)
        .ok_or_eyre(format!("end delimiter {} not found", &end_delimiter))?;
    let toml_end = toml_start + end_index;
    let toml_str = &content[toml_start..toml_end];

    debug!("converting {:?}", toml_str);
    let toml: BlogPostConfig = toml::from_str(toml_str)?;
    debug!("toml {:?}", &toml);

    let markdown_start = toml_end + end_delimiter.len();
    let markdown_content = content[markdown_start..].to_string();

    Ok((toml, markdown_content))
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
    fn render(self, config: &BlogPostConfig) -> Result<String, RenderError> {
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
