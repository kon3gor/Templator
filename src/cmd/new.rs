use std::env;
use std::io::stdin;

use termion::{event::Key, input::TermRead};

use crate::{
    api::TemplateSource, cli::CliController, cmd::api::Command, composite::CompositeSource,
    error::TemplatorError, Settings,
};

pub struct NewCommand {}

impl NewCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for NewCommand {
    fn process(&self, args: Vec<String>, settings: Settings) -> crate::api::TemplatorResult<()> {
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

        retriever.load_choice(selected_choice.clone(), args.get(0).cloned())?;

        Ok(())
    }
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
