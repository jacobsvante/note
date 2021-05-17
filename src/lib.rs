pub mod cli;
pub mod logging;
use std::{ffi::OsString, fs::File, io::Write, path::PathBuf};

use chrono::{Date, Local};

pub fn make_file_path(
    base_dir: PathBuf,
    date: Date<Local>,
    name: &str,
) -> Result<PathBuf, std::io::Error> {
    let file_dir = base_dir.join(date.format("%Y-%m-%d").to_string());

    std::fs::create_dir_all(&file_dir)?;

    let mut file_name = PathBuf::from(&name);
    file_name.set_extension("md");

    Ok(file_dir.join(file_name))
}

pub fn new(file_path: PathBuf, title: &str) -> Result<PathBuf, std::io::Error> {
    log::debug!("Creating note {:?}", &file_path);
    let mut file = File::create(&file_path)?;
    writeln!(file, "# {}", title)?; // Markdown header
    Ok(file_path)
}

pub fn edit(file_path: PathBuf) -> Result<(), std::io::Error> {
    log::debug!("Editing note {:?}", &file_path);
    file_path.metadata()?; // Ensure that file exists
    edit::edit_file(file_path)?;
    Ok(())
}

pub fn list(base_dir: PathBuf, date: Date<Local>) -> Result<Vec<OsString>, std::io::Error> {
    let file_dir = base_dir.join(date.format("%Y-%m-%d").to_string());
    let file_paths: Result<Vec<_>, std::io::Error> = std::fs::read_dir(file_dir)?
        .map(|res| res.map(|e| e.path().into_os_string()))
        .collect();
    let file_paths: Vec<_> = file_paths?
        .into_iter()
        .filter(|filename| filename.to_string_lossy().ends_with(".md"))
        .collect();
    Ok(file_paths)
}
