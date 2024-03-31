use std::env;

use crate::{api::TemplatorResult, error::TemplatorError, Settings};

use super::{help::HelpCommand, list::ListCommand, new::NewCommand};

pub trait Command {
    fn process(&self, args: Vec<String>, settings: Settings) -> TemplatorResult<()>;
}

pub fn process_args(settings: Settings) -> TemplatorResult<()> {
    let args: Vec<String> = env::args().collect();
    let cmd = args
        .get(1)
        .ok_or(TemplatorError::from("No args were provied"))?;

    let cmd_args = args[2..].to_vec();
    match cmd.as_str() {
        "new" => NewCommand::new().process(cmd_args, settings),
        "list" => ListCommand::new().process(cmd_args, settings),
        "help" => HelpCommand::new().process(cmd_args, settings),
        _ => Err(TemplatorError::from("Unknown command.")),
    }
}
