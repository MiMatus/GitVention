use std::error::Error;
use gitvention::{ validate::validate, cmd::{Args, Commands}};
use clap::{Parser};


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();


    match args.command {
        Commands::Validate(validationArgs) => {
            validate(&args.git_config,&validationArgs)
        }
    }

}