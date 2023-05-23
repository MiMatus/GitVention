use std::{ process::{Command, Output}, error::Error};

use crate::{cmd::GitConfig, error::GitVentionError};

pub trait Executor {
    fn exec(&self, args: &[&str]) -> Result<String, Box<dyn Error>>;
}

pub struct GitExecutor {
    repository_path: String,
}

impl GitExecutor{
    pub fn from_config(config: &GitConfig) -> Result<Self, Box<dyn std::error::Error>> {

        let repository_path = match &config.repository {
                Some(path) => path.to_owned(),
                None => String::from("./.git")
        };

        let executor = Self{repository_path};

        executor.exec(&["rev-parse", "--absolute-git-dir"])?;

        Ok(executor)
    }

}

impl Executor for GitExecutor {
    fn exec(&self, args: &[&str]) -> Result<String, Box<dyn Error>>{

        let git_dir_arg = &format!("--git-dir={}", self.repository_path) as  &str;

        let git_args = [&[git_dir_arg], args].concat();

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
}