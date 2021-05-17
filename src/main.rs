use std::{io::Write, os::unix::prelude::OsStrExt};

use note::{cli, logging};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = <cli::Cli as clap::Clap>::parse();
    logging::init(&opts.log_level);

    match opts.subcmd {
        cli::SubCommand::New(subopts) => {
            let file_path =
                note::make_file_path(opts.base_directory, opts.date.date(), &subopts.name)?;
            note::new(file_path.clone(), &subopts.name)?;
            note::edit(file_path)?;
        }
        cli::SubCommand::Edit(subopts) => {
            let file_path =
                note::make_file_path(opts.base_directory, opts.date.date(), &subopts.name)?;
            note::edit(file_path)?;
        }
        cli::SubCommand::List(_subopts) => {
            let mut stdout = std::io::stdout();
            for n in note::list(opts.base_directory, opts.date.date())? {
                stdout.write_all(n.as_bytes())?;
                stdout.write_all(b"\n")?;
            }
        }
    }
    Ok(())
}
