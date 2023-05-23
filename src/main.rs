use std::error::Error;
use gitvention::{ validate::validate, cmd::{Args, Commands}, git::{search::Search, executor::GitExecutor}};
use clap::{Parser};


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let search = Search{executor: GitExecutor::from_config(&args.git_config)?};

    match args.command {
        Commands::Validate(validation_args) => {
            validate(search,&validation_args, &args.conventions)
        }
    }

}