use std::{fmt::{Formatter, Result, Display}, error::Error};


#[derive(Debug, Clone)]
pub struct GitVentionError(pub String);

impl Error for GitVentionError{}

impl Display for GitVentionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
