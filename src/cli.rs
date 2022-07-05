use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDate, ParseError};

fn parse_notes_dir(src: &str) -> Result<PathBuf, String> {
    let src = shellexpand::tilde(src).to_string();
    let p = PathBuf::from(src);
    match p.metadata() {
        Ok(_) => Ok(p),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Notes directory {p:?} does not exist");
            Ok(p)
        }
        Err(error) => Err(error.to_string()),
    }
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

#[derive(clap::Parser)]
#[clap(version = "0.1")]
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

#[derive(clap::Subcommand)]
pub enum SubCommand {
    /// Create a new note and edit it in your default editor
    ///
    /// The env var VISUAL or EDITOR is used to detect the correct editor.
    /// Date and base directory is specified by the parent command
    New { name: String },
    /// Edit an existing note it in your default editor
    ///
    /// The env var VISUAL or EDITOR is used to detect the correct editor.
    /// Date and base directory is specified by the parent command/
    Edit { name: String },
    /// List all notes that exist
    List,
}
