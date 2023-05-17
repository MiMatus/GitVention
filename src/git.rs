use std::{ process::{Command, Output}, error::Error};

use crate::{cmd::GitConfig, error::GitVentionError};


pub struct Git {
    path: String,
}


impl Git{
    pub fn from_config(config: &GitConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let path = git_path(config)?;
        Ok(Self {path})
    }

    fn exec(&self, args: &[&str]) -> Result<String, Box<dyn Error>>{

        let git_path: &[&str] = &[&format!("--git-dir={}", self.path)];
        let git_args = [git_path , args].concat();

        let Output {status, stderr, stdout} = Command::new("git")
            .args(&git_args)
            .output()?;
    
        if !status.success() {
            let git_errors = String::from_utf8(stderr)?;
    
            let e = Box::new(GitVentionError(format!(
                "Unable to execute git cmd calling: `{}` returned: `{}`", git_args.join("|"), git_errors
            )));
    
            return Err(e);
        }
        
        Ok(String::from_utf8(stdout)?)
    }

    pub fn commit_hashes(&self, range: &str) -> Result<Vec<String>, Box<dyn Error>> {
    
        let hashes = self.exec(&[
            "log", 
            "--pretty=format:%h", 
            range
        ])?.lines().map(|hash| hash.to_string()).collect();

        Ok(hashes)
    }

    pub fn commit_message(&self, hash: &str) -> Result<String, Box<dyn Error>> {

        let message = self.exec(&[
            "log", 
            "-n 1", 
            "--pretty=format:%s", 
            hash
        ])?;
                
        Ok(message)
    }

}

fn git_path(config: &GitConfig) -> Result<String, Box<dyn std::error::Error>> {

    let git_dir_arg = match &config.repository {
        Some(repo) => format!("--git-dir={}", repo),
        None => String::new(),
    };

    let Output {status, stderr, stdout} = Command::new("git")
        .args([&git_dir_arg, "rev-parse", "--absolute-git-dir"])
        .output()?;


    if !status.success() {
        let git_errors = String::from_utf8(stderr)?;

        let e = Box::new(GitVentionError(format!(
            "Unable to find path to git folder error: {}", git_errors
        )));

        return Err(e);
    }

    let path = String::from_utf8(stdout)?.trim_end().to_owned();


    Ok(path)
}