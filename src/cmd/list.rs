use std::path::PathBuf;

use crate::{api::TemplateSource, composite::CompositeSource};

use super::api::Command;

pub struct ListCommand {}

impl ListCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ListCommand {
    fn process(
        &self,
        _args: Vec<String>,
        settings: crate::Settings,
    ) -> crate::api::TemplatorResult<()> {
        let retriever = CompositeSource::new(PathBuf::from("/"), settings);
        let choices = retriever.get_choices()?;
        println!("Available templates:");
        for pair in choices.iter().enumerate() {
            println!("{}. \"{}\"", pair.0 + 1, pair.1);
        }
        Ok(())
    }
}
