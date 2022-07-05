use clap::Parser;
use note::{cli, logging};
use std::{io::Write, os::unix::prelude::OsStrExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = cli::Cli::parse();
    logging::init(&opts.log_level);

    match opts.subcmd {
        cli::SubCommand::New { name } => {
            let file_path = note::make_file_path(opts.base_directory, opts.date.date(), &name)?;
            note::new(file_path.clone(), &name)?;
            note::edit(file_path)?;
        }
        cli::SubCommand::Edit { name } => {
            let file_path = note::make_file_path(opts.base_directory, opts.date.date(), &name)?;
            note::edit(file_path)?;
        }
        cli::SubCommand::List => {
            let mut stdout = std::io::stdout();
            for n in note::list(opts.base_directory, opts.date.date())? {
                stdout.write_all(n.as_bytes())?;
                stdout.write_all(b"\n")?;
            }
        }
    }
    Ok(())
}
