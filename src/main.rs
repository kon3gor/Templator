mod api;
mod cli;
mod composite;
mod error;
mod local;
mod github;

use error::TemplatorError;
use serde::Deserialize;
use std::env::VarError;
use std::io::stdin;
use std::path::PathBuf;
use std::{env, fs, usize};
use termion::event::Key;
use termion::input::TermRead;

use crate::api::TemplateSource;
use crate::cli::CliController;
use crate::composite::CompositeSource;

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
    let cwd = env::current_dir()?;

    let retriever = CompositeSource::new(cwd, settings);
    let choices = retriever.get_choices()?.into_boxed_slice();
    let cnt = choices.len();

    let mut selected: usize = 0;
    let mut cli = CliController::new();

    draw_choices(&mut cli, &choices, selected);
    cli.move_up(cnt);

    let stdin = stdin();
    let mut exit = false;
    for c in stdin.keys() {
        match c? {
            Key::Char('j') => {
                if selected == cnt - 1 {
                    continue;
                }
                cli.move_up(selected);
                selected += 1;
                draw_choices(&mut cli, &choices, selected);
                cli.move_up(cnt - selected);
            }
            Key::Char('k') => {
                if selected == 0 {
                    continue;
                }
                cli.move_up(selected);
                selected -= 1;
                draw_choices(&mut cli, &choices, selected);
                cli.move_up(cnt - selected);
            }
            Key::Char('\n') => break,
            Key::Char('q') => {
                //todo: Add help message with this shit
                exit = true;
                break;
            }
            _ => (),
        }

        cli.flush();
    }

    if exit {
        return Ok(());
    }

    cli.move_down(cnt - selected);
    let selected_choice = choices
        .get(selected)
        .ok_or(TemplatorError::from("Index out of bounds"))?;
    cli.print(format!(
        "You have selected: {}\r\n",
        selected_choice.clone()
    ));
    cli.flush();

    retriever.load_choice(selected_choice.clone())?;

    return Ok(());
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

fn draw_choices(cli: &mut CliController, choices: &[String], selected: usize) {
    for pair in choices.iter().enumerate() {
        cli.clear_line();
        let choice = pair.1;
        if pair.0 == selected {
            cli.print(format!("> {}\r\n", choice));
        } else {
            cli.print(format!("{}\r\n", choice));
        }
    }
}
