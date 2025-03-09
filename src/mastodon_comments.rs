use std::fs;

use handlebars::Handlebars;
use maud::{html, Markup, PreEscaped};
use toml::Table;


pub fn mastodon_comments(config: &Table) -> Markup {
    let script = fs::read_to_string(std::env::current_dir().unwrap().join("src/script.js")).expect("Unable to read file");

    let html = html! {
        script type="text/javascript" {
            (PreEscaped(script))
        }

        button onclick={"load_comments('{{extra.postid}}')" } {
            "LOAD"
        }
        a id="comments-view" {
            "View"
        }
    }.into_string();

    let out = Handlebars::new().render_template(&html, config).unwrap();

    PreEscaped(
        out
    )
}


// {"date": Datetime(Datetime { date: Some(Date { year: 2025, month: 2, day: 27 }), time: None, offset: None }), "extra": Table({"miku_img": String("miku_jump"), "miku_q": String("Hit the GRTTy!"), "postid": Integer(114075174358638781)}), "title": String("GRTTy Stack: gRPC Rust React Typescript")}

