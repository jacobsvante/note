pub mod cli;
pub mod logging;
use std::{fs::File, io::Write, path::PathBuf};

use chrono::{Date, Local};

pub fn make_file_path(base_dir: PathBuf, date: Date<Local>, name: &str) -> Result<PathBuf, std::io::Error> {
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
