use super::api::Command;

pub struct HelpCommand {}

impl HelpCommand {
    pub fn new() -> Self {
        Self {}
    }
}

const HELP_MSG: &'static str =
    "Templator is a simple tool for creating projects from predefined templates.
 
Usage: templ CMD [...args]

Commands:
    new  Creates a new project from the chosen template.
    list Lists all available templates
    help Prints this message
";

impl Command for HelpCommand {
    fn process(
        &self,
        _args: Vec<String>,
        _settings: crate::Settings,
    ) -> crate::api::TemplatorResult<()> {
        println!("{}", HELP_MSG);
        Ok(())
    }
}
