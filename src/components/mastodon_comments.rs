use std::fs;

use handlebars::Handlebars;
use log::debug;
use maud::{html, Markup, PreEscaped};

use crate::BlogPostConfig;

pub fn mastodon_comments(config: &BlogPostConfig) -> Markup {
    if !config.extra.comments {
        return html! {
            div { "comments are disabled" }
        };
    }

    let script = fs::read_to_string(std::env::current_dir().unwrap().join("src/script.js"))
        .expect("Unable to read file");

    debug!("config.contains_key extra.postid {:?}", config.extra.postid);

    let html = html! {
        script type="text/javascript" {
            (PreEscaped(script))
        }

        @if !config.extra.postid.is_empty() {
            button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={"load_comments('{{extra.postid}}')" } {
                "Load Mastodon Comments"
            }
        }
    }.into_string();

    let out = Handlebars::new().render_template(&html, config).unwrap();

    PreEscaped(out)
}
