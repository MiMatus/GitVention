use std::{process::{Command, Output}};
use error::GitVentionError;

pub mod error;
pub mod validate;
pub mod cmd;
pub mod git;

pub fn git_path() -> Result<String, Box<dyn std::error::Error>> {

    let Output {status, stderr, stdout} = Command::new("git")
        .args(["rev-parse", "--absolute-git-dir"])
        .output()?;


    if !status.success() {
        let git_errors = String::from_utf8(stderr)?;

        let e = Box::new(GitVentionError(format!(
            "Unable to find path to git folder error: {}", git_errors
        )));

        return Err(e);
    }

    let path = String::from_utf8(stdout)?;

    Ok(path)
}
