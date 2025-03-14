use blog::{
    copy_static_content, read_all_files, render_posts_to_html, save_file, save_html_posts,
    BlogPostConfig,
};
use color_eyre::Report;

use blog::{about, index, template};
use std::path::Path;

use clap::{Arg, Command};
use log::LevelFilter;
use simple_logger::SimpleLogger;

/// On error handling: This is a program designed to be used via command line by
/// a technical audience. This is why we bubble the errors to the main function
/// and use `color_eyre::Report` to generate an error report on the console.
fn main() -> Result<(), Report> {
    init_loglevel()?;
    color_eyre::install()?;

    let out_dir: &Path = Path::new("docs");
    let public_dir: &Path = Path::new("public");
    let content_dir: &Path = Path::new("src/content");

    let md_posts = read_all_files(content_dir, "md")?;
    let html_posts: Vec<(String, BlogPostConfig, String)> = render_posts_to_html(&md_posts)?;
    save_html_posts(&html_posts, out_dir)?;

    save_file(
        &template(index(html_posts)?).into_string(),
        out_dir,
        "index.html",
    )?;
    save_file(&template(about()).into_string(), out_dir, "about.html")?;

    copy_static_content(Path::new(public_dir), Path::new(out_dir))?;

    println!("Built site OK!");
    Ok(())
}

fn init_loglevel() -> Result<(), Report> {
    let matches = Command::new("chuub")
        .version("1.0")
        .author("chuu <j@chuu.dev>")
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
