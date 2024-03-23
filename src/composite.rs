use std::path::PathBuf;

use crate::{
    api::{TemplateSource, TemplatorResult},
    local::LocalSource,
    Settings, StorageType,
};

pub struct CompositeSource {
    source: Box<dyn TemplateSource>,
}

impl CompositeSource {
    pub fn new(cwd: PathBuf, settings: Settings) -> Self {
        let source = match settings.storage_type {
            StorageType::Local => LocalSource::new(settings.storage_uri.clone(), cwd),
            StorageType::Git => todo!(),
        };
        Self {
            source: Box::new(source),
        }
    }
}

impl TemplateSource for CompositeSource {
    fn get_choices(&self) -> TemplatorResult<Vec<String>> {
        return self.source.get_choices();
    }

    fn load_choice(&self, choice: String) -> TemplatorResult<()> {
        return self.source.load_choice(choice);
    }
}