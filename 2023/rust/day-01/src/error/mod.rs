use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AocError {
    IoError
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for AocError {}