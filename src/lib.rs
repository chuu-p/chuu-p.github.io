use color_eyre::Report;

use handlebars::Handlebars;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use pulldown_cmark::{html::push_html, Parser};
use std::{
    error::Error,
    fs::{self, DirEntry, FileType},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    sync::LazyLock,
};
use toml::Table;

/// Copies the contents of `src` to `dst`, recursively.
/// 
/// # Examples
///
/// ## Basic Copy
/// ```
/// use std::fs::{self, File};
/// use std::path::Path;
/// use tempfile::tempdir;
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
pub fn copy_static_content(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());

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
