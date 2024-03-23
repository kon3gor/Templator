use std::env::VarError;
use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct TemplatorError {
    msg: String,
}

impl TemplatorError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for TemplatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for TemplatorError {}

impl From<toml::de::Error> for TemplatorError {
    fn from(value: toml::de::Error) -> Self {
        return TemplatorError::new(value.to_string());
    }
}

impl From<io::Error> for TemplatorError {
    fn from(value: io::Error) -> Self {
        return TemplatorError::new(value.to_string());
    }
}

impl From<VarError> for TemplatorError {
    fn from(value: VarError) -> Self {
        return TemplatorError::new(value.to_string());
    }
}

impl From<&'static str> for TemplatorError {
    fn from(value: &'static str) -> Self {
        return TemplatorError::new(value.to_string());
    }
}
