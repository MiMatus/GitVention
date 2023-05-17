use std::process::{Command, Output};
use std::error::Error;
use crate::cmd::{ValidationArgs, GitConfig};
use crate::git::Git;
use regex::Regex;


fn validate_message(message: &str) -> bool {
    let re = Regex::new(r"^(\w:)+.*$").unwrap();
    return re.is_match(message);
}


pub fn validate(git_config: &GitConfig, args: &ValidationArgs) -> Result<(), Box<dyn Error>> {
    let git = Git::from_config(git_config)?;

    let hashes = git.commit_hashes(&args.target)?;


    for hash in hashes {
        let message = git.commit_message(&hash)?;

        if validate_message(&message){
            println!("Commit: `{}` is VALID", message)
        } else {
            eprintln!("Commit: `{}` is INVALID", message)
        }
    }

    Ok(())
}
