mod api;
mod cli;
mod cmd;
mod composite;
mod error;
mod github;
mod local;

use error::TemplatorError;
use serde::Deserialize;
use std::env::VarError;
use std::path::PathBuf;
use std::{env, fs};

const SETTINGS_ENV_VAR: &'static str = "TEMPLATOR_SETTINGS";
const HOME_ENV_VAR: &'static str = "HOME";
const DEFAULT_LOCATION: &'static str = ".templator/settings.toml";

#[derive(Debug, Deserialize)]
pub enum StorageType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "git")]
    Git,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(rename = "storage")]
    pub storage_uri: String,
    #[serde(rename = "type")]
    pub storage_type: StorageType,
}

fn main() -> Result<(), TemplatorError> {
    let settings = get_settings()?;
    return crate::cmd::api::process_args(settings);
}

fn get_settings_location() -> Result<PathBuf, VarError> {
    let custom_location = env::var(SETTINGS_ENV_VAR);
    if let Ok(location) = custom_location {
        return Ok(PathBuf::from(location));
    }

    let home_env = env::var(HOME_ENV_VAR)?;
    return Ok(PathBuf::from(home_env).join(DEFAULT_LOCATION));
}

fn get_settings() -> Result<Settings, TemplatorError> {
    let location = get_settings_location()?;
    let content = fs::read_to_string(location)?;
    return toml::from_str(&content).map_err(TemplatorError::from);
}
