use std::{  error::Error};

use super::{executor::Executor, commit::Commit};


pub trait Searcher {
    fn commit_hashes(&self, range: &str) -> Result<Vec<String>, Box<dyn Error>>;
    fn commit_by_hash(&self, hash: &str) -> Result<Commit, Box<dyn Error>>;
}

pub struct Search<Exec: Executor> {
    pub executor: Exec
}

impl<Exec: Executor> Searcher for Search<Exec>{
    fn commit_hashes(&self, range: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let hashes = self.executor.exec(
            &[
                "log", 
                "--pretty=format:%h", 
                range
            ]
        )?.lines().map(|hash| hash.to_string()).collect();

        Ok(hashes)
    }
    fn commit_by_hash(&self, hash: &str) -> Result<Commit, Box<dyn Error>> {
        let message = self.executor.exec(
            &[
            "log", 
            "-n 1", 
            "--pretty=format:%s", 
            hash
        ])?;
                
        Ok(Commit{message})
    }
}
