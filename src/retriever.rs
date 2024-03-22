use std::path::PathBuf;

use crate::{api::TemplateSource, local::LocalSource, Settings, StorageType};

pub struct TemplateRetriever {
    source: Box<dyn TemplateSource>,
}

impl TemplateRetriever {
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

impl TemplateSource for TemplateRetriever {
    fn get_choices(&self) -> Vec<String> {
        return self.source.get_choices();
    }

    fn load_choice(&self, choice: String) -> bool {
        return self.source.load_choice(choice);
    }
}
