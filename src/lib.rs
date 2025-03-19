use color_eyre::Report;
use components::blog_post::get_header;
pub use components::{about, blog_post, index, mastodon_comments, template};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, DirEntry},
    io::{self, ErrorKind},
    path::Path,
};
use toml::value::Date;

mod components;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogPostConfig {
    pub title: String,
    pub date: Date,
    pub extra: BlogPostConfigExtra,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlogPostConfigExtra {
    pub comments: bool,
    pub postid: String,
    pub miku_img: String,
    pub miku_q: String,
}

/// Saves the given content to a file in the specified output directory.
///
/// # Examples
///
/// Successful write:
/// ```
/// use std::fs;
/// use tempfile::tempdir;
/// use blog::save_file;
///
/// const FILE_NAME: &str = "example.txt";
/// const CONTENT: &str = "Hello, world!";
///
/// let dir = tempdir().unwrap();
/// let file_path = dir.path().join(FILE_NAME);
///
/// println!("file_path {:?}", &file_path);
/// println!("save_file {:?} {:?} {:?}", &CONTENT, &dir.path(), &FILE_NAME);
/// save_file(CONTENT, dir.path(), FILE_NAME).unwrap();
///
/// let content = fs::read_to_string(&file_path).unwrap();
/// assert_eq!(content, CONTENT);
/// ```
///
/// Directory does not exist:
/// ```
/// use blog::save_file;
/// use std::path::Path;
///
/// let result = save_file("Hello!", Path::new("/non/existent/dir"), "file.txt");
/// assert!(result.is_err());
/// ```
///
/// Invalid file name:
/// ```
/// #[cfg(target_os = "windows")]
/// {
///     let result = save_file("Hello!", "C:\\", "<invalid:name>");
///     assert!(result.is_err());
/// }
/// ```
pub fn save_file(content: &str, out_dir: impl AsRef<Path>, file_name: &str) -> io::Result<()> {
    let target = out_dir.as_ref().join(file_name);
    println!("write {:?}", &target);
    fs::write(&target, content)?;
    Ok(())
}

/// Reads all files with the given extension from the specified path.
///
/// # Examples
///
/// Successful read:
/// ```
/// use std::fs;
/// use tempfile::tempdir;
/// use blog::read_all_files;
/// use color_eyre::{eyre::OptionExt, Report};
///
/// let dir = tempdir().unwrap();
/// fs::write(dir.path().join("file1.txt"), "Content 1").unwrap();
/// fs::write(dir.path().join("file2.txt"), "Content 2").unwrap();
///
/// let t = dir.path();
/// let files = read_all_files(t, "txt");
/// let files = files.unwrap();
///
/// assert_eq!(files.len(), 2);
/// assert!(files.iter().any(|(name, content)| name == "file1.html" && content == "Content 1"));
/// assert!(files.iter().any(|(name, content)| name == "file2.html" && content == "Content 2"));
/// ```
///
/// No matching files:
/// ```
/// use tempfile::tempdir;
/// use blog::read_all_files;
///
/// let binding = tempdir().unwrap();
/// let dir = binding.path();
/// let files = read_all_files(dir, "md").unwrap();
/// assert!(files.is_empty());
/// ```
///
/// Invalid path:
/// ```
/// use blog::read_all_files;
/// use std::path::Path;
///
/// let result = read_all_files(Path::new("/non/existent/path"), "txt");
/// assert!(result.is_err());
/// ```
pub fn read_all_files(
    path: impl AsRef<Path>,
    extension: &str,
) -> io::Result<Vec<(String, String)>> {
    let entries = fs::read_dir(&path)?
        .filter_map(|res| {
            let de = res.ok()?;
            let ft = &de.file_type().ok()?;
            if ft.is_file() && de.path().extension().is_some_and(|ext| ext == extension) {
                Some(Ok::<DirEntry, io::Error>(de))
            } else {
                None
            }
        })
        .collect::<Result<Vec<DirEntry>, io::Error>>()?;

    debug!("{:?}", &entries);

    let res = entries
        .into_iter()
        .map(|de| {
            let path = path.as_ref().join(de.file_name());
            let mut html_path = path.clone();
            html_path.set_extension("html");

            let file_name = html_path
                .file_name()
                // .ok_or_else(|| "filename error")?
                .ok_or_else(|| io::Error::new(ErrorKind::Other, "filename error"))?
                .to_str()
                .ok_or_else(|| io::Error::new(ErrorKind::Other, "to_str error"))?
                .to_string();

            let content = fs::read_to_string(&path)?;

            Ok((file_name, content))
        })
        .collect::<Result<Vec<(String, String)>, _>>();

    debug!("{:?}", &res);

    res
}

pub fn render_posts_to_html(
    posts: &[(String, String)],
) -> Result<Vec<(String, BlogPostConfig, String)>, Report> {
    posts
        .iter()
        .map(|(path, post)| {
            let (table, rendered_page) = blog_post(post.to_string())?;
            Ok::<(String, BlogPostConfig, String), Report>((path.to_owned(), table, rendered_page))
        })
        .collect::<Result<Vec<(String, BlogPostConfig, String)>, Report>>()
}

/// Saves provided HTML content as `.html` files in the specified output directory.
///
/// Each tuple in `posts` contains a file path (String), a mock `Table` config,
/// and the page content as a string. The path will have its extension changed to `.html`.
///
/// # Errors
/// Returns an `io::Result` error if file writing fails.
pub fn save_html_posts(
    posts: &[(String, BlogPostConfig, String)],
    output_dir: impl AsRef<Path>,
) -> io::Result<()> {
    for (path, _config, page) in posts {
        let mut target = output_dir.as_ref().join(path);
        target.set_extension("html");
        fs::write(&target, page)?;
    }
    Ok(())
}

/// Copies the contents of `src` to `dst`, recursively.
///
/// # Examples
///
/// ## Basic Copy
/// ```
/// use std::fs::{self, File};
/// use std::path::Path;
/// use tempfile::tempdir;
/// use blog::copy_static_content;
///
/// let src_dir = tempdir().unwrap();
/// let dst_dir = tempdir().unwrap();
///
/// let src_file = src_dir.path().join("example.txt");
/// let dst_file = dst_dir.path().join("example.txt");
///
/// File::create(&src_file).unwrap();
/// copy_static_content(src_dir.path(), dst_dir.path()).unwrap();
///
/// assert!(dst_file.exists());
/// ```
///
/// ## Nested Directory Copy
/// ```
/// use std::fs::{self, File};
/// use tempfile::tempdir;
/// use blog::copy_static_content;
///
/// let src_dir = tempdir().unwrap();
/// let nested_dir = src_dir.path().join("nested");
/// fs::create_dir(&nested_dir).unwrap();
///
/// let nested_file = nested_dir.join("file.txt");
/// File::create(&nested_file).unwrap();
///
/// let dst_dir = tempdir().unwrap();
/// copy_static_content(src_dir.path(), dst_dir.path()).unwrap();
///
/// assert!(dst_dir.path().join("nested/file.txt").exists());
/// ```
///
/// ## Skip Unchanged Files
/// ```
/// use std::fs::{self, File};
/// use std::io::Write;
/// use tempfile::tempdir;
/// use std::time::{SystemTime, Duration};
/// use blog::copy_static_content;
///
/// let src_dir = tempdir().unwrap();
/// let dst_dir = tempdir().unwrap();
///
/// let src_file = src_dir.path().join("example.txt");
/// let dst_file = dst_dir.path().join("example.txt");
///
/// let mut file = File::create(&src_file).unwrap();
/// writeln!(file, "Original content").unwrap();
///
/// File::create(&dst_file).unwrap().set_modified(SystemTime::now() + Duration::from_secs(3600)).unwrap();
///
/// copy_static_content(src_dir.path(), dst_dir.path()).unwrap();
///
/// // The original file should *not* overwrite the newer file
/// assert!(fs::read_to_string(dst_file).unwrap().is_empty());
/// ```
/// see: https://stackoverflow.com/a/65192210/26371953
pub fn copy_static_content(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.as_ref().join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_static_content(&entry_path, &target_path)?;
        } else if should_copy_file(&entry_path, &target_path)? {
            fs::copy(&entry_path, &target_path)?;
        }
    }

    Ok(())
}

/// Determines if a file should be copied based on its modification time.
///
/// # Arguments
///
/// * `src` - Path to the source file.
/// * `dst` - Path to the destination file.
///
/// # Returns
///
/// * `Ok(true)` if the destination file is missing or older than the source.
/// * `Ok(false)` if the destination file is newer or has the same modification time.
/// * `Err` if any I/O error occurs during metadata retrieval.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use std::fs;
/// use std::time::{SystemTime, Duration};
/// use tempfile::NamedTempFile;
/// use blog::should_copy_file;
///
/// fn mock_file(path: &Path, modified: SystemTime) {
///     fs::write(path, "dummy data").unwrap();
///     filetime::set_file_mtime(path, filetime::FileTime::from_system_time(modified)).unwrap();
/// }
///
/// fn main() -> std::io::Result<()> {
///     let src = NamedTempFile::new()?.into_temp_path();
///     let dst = NamedTempFile::new()?.into_temp_path();
///
///     mock_file(&src, SystemTime::now());
///     mock_file(&dst, SystemTime::now() - Duration::from_secs(3600));
///
///     assert!(should_copy_file(&src, &dst)?);
///     Ok(())
/// }
/// ```
pub fn should_copy_file(src: &Path, dst: &Path) -> io::Result<bool> {
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
