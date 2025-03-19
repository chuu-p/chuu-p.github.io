use std::fs;

use color_eyre::Report;
use handlebars::Handlebars;
use crate::{get_header, components::BlogPost, components::Markdown};

pub fn about() -> Result<String, Report> {
    let path = "src/content/static/about.md";
    let content = fs::read_to_string(&path)?;

    let (table, content) = get_header(&content)?;

    let html = Markdown(&content).render();
    let out = BlogPost(&html).render(&table);

    let rendered_page = Handlebars::new().render_template(&out?, &table)?;

    Ok(rendered_page)
}
