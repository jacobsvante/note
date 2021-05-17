use note::{cli, logging};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = <cli::Cli as clap::Clap>::parse();
    logging::init(&opts.log_level);

    match opts.subcmd {
        cli::SubCommand::New(subopts) => {
            let file_path = note::make_file_path(opts.base_directory, opts.date.date(), &subopts.name)?;
            note::new(file_path.clone(), &subopts.name)?;
            note::edit(file_path)?;
        },
        cli::SubCommand::Edit(subopts) => {
            let file_path = note::make_file_path(opts.base_directory, opts.date.date(), &subopts.name)?;
            note::edit(file_path)?;
        }
    }
    Ok(())
}
