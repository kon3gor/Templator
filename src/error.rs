use std::env::VarError;
use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct MyError {}

impl MyError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "my error occured")
    }
}

impl Error for MyError {}

impl From<toml::de::Error> for MyError {
    fn from(value: toml::de::Error) -> Self {
        return MyError::new();
    }
}

impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        return MyError::new();
    }
}

impl From<VarError> for MyError {
    fn from(value: VarError) -> Self {
        return MyError::new();
    }
}
