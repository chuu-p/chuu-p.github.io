use std::path::Path;

use blog::{
    copy_static_content, read_all_files, render_posts_to_html, save_file, save_html_posts, BlogPostConfig
};
use color_eyre::Report;

use blog::{about, index, template};

use clap::{Arg, Command};
use log::LevelFilter;
use simple_logger::SimpleLogger;

/// On error handling: This is a program designed to be used via command line by
/// a technical audience. This is why we bubble the errors to the main function
/// and use `color_eyre::Report` to generate an error report on the console.
fn main() -> Result<(), Report> {
    init_loglevel()?;
    color_eyre::install()?;

    const OUT_DIR: &str = "docs";
    const PUBLIC_DIR: &str = "public";
    const CONTENT_DIR: &str = "src/content";

    let md_posts = read_all_files(Path::new(CONTENT_DIR), "md")?;
    let html_posts: Vec<(String, BlogPostConfig, String)> = render_posts_to_html(&md_posts)?;
    save_html_posts(&html_posts, Path::new(OUT_DIR))?;

    save_file(
        &template(index(html_posts)?).into_string(),
        Path::new(OUT_DIR),
        "index.html",
    )?;
    save_file(
        &template(about()?).into_string(),
        Path::new(OUT_DIR),
        "about.html",
    )?;

    copy_static_content(PUBLIC_DIR, OUT_DIR)?;

    println!("Built site OK!");
    Ok(())
}

fn init_loglevel() -> Result<(), Report> {
    let matches = Command::new("chuub")
        .version("1.0")
        .author("chuu <chuu801@pm.me>")
        .about("A simple static site generator")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(clap::ArgAction::Count),
        )
        .get_matches();

    let log_level = match matches.get_count("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    SimpleLogger::new().with_level(log_level).init()?;

    Ok(())
}
