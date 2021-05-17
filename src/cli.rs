use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDate, ParseError};
use clap::{AppSettings, Clap};

fn parse_notes_dir(src: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let src = shellexpand::tilde(src).to_string();
    let p = PathBuf::from(src);
    p.metadata()?;
    Ok(p)
}

fn parse_date(src: &str) -> Result<DateTime<Local>, ParseError> {
    if src == "TODAY" {
        Ok(Local::now())
    } else {
        let dt = NaiveDate::parse_from_str(src, "%Y-%m-%d")?.and_hms(0, 0, 0);
        Ok(DateTime::<Local>::from_utc(
            dt,
            Local::now().offset().to_owned(),
        ))
    }
}

#[derive(Clap)]
#[clap(version = "0.1", setting = AppSettings::ColoredHelp)]
pub struct Cli {
    #[clap(short, long, default_value = "info")]
    pub log_level: String,
    /// The base directory in which the note will be looked for
    #[clap(short, long, default_value = "~/notes", parse(try_from_str = parse_notes_dir))]
    pub base_directory: PathBuf,
    /// The date for which the note will be looked for, should be in ISO-format YYYY-mm-dd
    #[clap(short, long, default_value = "TODAY", parse(try_from_str = parse_date))]
    pub date: DateTime<Local>,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    New(HandleNew),
    Edit(HandleEdit),
}

/// Create a new note and edit it in your default editor
///
/// The env var VISUAL or EDITOR is used to detect the correct editor.
/// Date and base directory is specified by the parent command
#[derive(Clap)]
pub struct HandleNew {
    #[clap()]
    pub name: String,
}

/// Edit an existing note it in your default editor
///
/// The env var VISUAL or EDITOR is used to detect the correct editor.
/// Date and base directory is specified by the parent command/
#[derive(Clap)]
pub struct HandleEdit {
    #[clap()]
    pub name: String,
}
